use serde_json::Value;

use crate::shared::Id;
use crate::stack::{Output, Resource, Stack};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum StackBuilderError {
    MissingPermissionsForRole(Vec<String>),
    DuplicateIds(Vec<String>),
    DuplicateResourceIds(Vec<String>),
    ResourceSpecificIssues(Vec<String>),
}

impl Display for StackBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StackBuilderError::MissingPermissionsForRole(info) => {
                let gathered_info = info.join(";");
                f.write_fmt(format_args!(
                    "one or more roles seem to be missing permission to access services: `{}`?",
                    gathered_info
                ))
            }
            StackBuilderError::DuplicateIds(info) => {
                let gathered_info = info.join(";");
                f.write_fmt(format_args!(
                    "ids should be unique, but the following duplicates were detected: `{}`",
                    gathered_info
                ))
            }
            StackBuilderError::ResourceSpecificIssues(info) => {
                let gathered_info = info.join(";");
                f.write_fmt(format_args!(
                    "resource specific issues detected: `{}`",
                    gathered_info
                ))
            }
            StackBuilderError::DuplicateResourceIds(info) => {
                let gathered_info = info.join(";");
                f.write_fmt(format_args!(
                    "duplicate resource ids detected (`{}`), rerunning this command should generate new ones",
                    gathered_info
                ))
            }
        }
    }
}

impl Error for StackBuilderError {}

/// Builder for CloudFormation stacks.
///
/// Collects resources and manages their relationships.
/// Might validate whether IAM roles are missing permissions for AWS services they need to access, based on Cargo.toml dependencies.
///
/// # Example
///
/// ```rust
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::sqs::QueueBuilder;
/// use rusty_cdk_core::wrappers::*;
///
/// let mut stack_builder = StackBuilder::new();
///
/// // Add resources to the stack
/// let queue = QueueBuilder::new("my-queue")
///     .standard_queue()
///     .build(&mut stack_builder);
///
/// // Add tags to the stack
/// stack_builder = stack_builder
///     .add_tag("Environment", "Production")
///     .add_tag("Owner", "Team");
///
/// // Build the stack
/// let stack = stack_builder.build().expect("Stack to build successfully");
/// ```
pub struct StackBuilder {
    resources: Vec<Resource>,
    tags: Vec<(String, String)>,
    outputs: Vec<(String, Value)>,
}

impl Default for StackBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StackBuilder {
    pub fn new() -> Self {
        Self {
            resources: vec![],
            tags: vec![],
            outputs: vec![],
        }
    }

    pub fn add_resource<T: Into<Resource>>(&mut self, resource: T) {
        let resource = resource.into();
        self.resources.push(resource);
    }

    pub fn add_tag<T: Into<String>>(mut self, key: T, value: T) -> Self {
        self.tags.push((key.into(), value.into()));
        self
    }

    pub fn add_output<T: Into<String>>(mut self, name: T, value: Value) -> Self {
        self.outputs.push((name.into(), value));
        self
    }

    pub(crate) fn get_resource(&mut self, id: &Id) -> Option<&mut Resource> {
        self.resources.iter_mut().find(|v| &v.get_id() == id)
    }

    /// Builds the stack and validates all resources.
    ///
    /// Might return an error if:
    /// - there are duplicate ids
    /// - IAM roles are missing permissions for AWS services they need to access (only when Cargo.toml dependencies were passed in)
    /// - Too many actions are specified for an alarm
    pub fn build(self) -> Result<Stack, StackBuilderError> {    
        let (ids, resource_ids) = self
            .resources
            .iter()
            .map(|r| (r.get_id().to_string(), r.get_resource_id().to_string()))
            .collect::<(Vec<_>, Vec<_>)>();

        let duplicate_ids = Self::check_for_duplicate_ids(ids);
        let resource_ids = Self::check_for_duplicate_ids(resource_ids);

        if !duplicate_ids.is_empty() {
            return Err(StackBuilderError::DuplicateIds(duplicate_ids));
        }
        if !resource_ids.is_empty() {
            return Err(StackBuilderError::DuplicateResourceIds(resource_ids));
        }

        let roles_with_potentially_missing_services: Vec<_> = self.check_for_roles_with_missing_permissions();

        if !roles_with_potentially_missing_services.is_empty() {
            return Err(StackBuilderError::MissingPermissionsForRole(
                roles_with_potentially_missing_services,
            ));
        }
        
        let resource_specific_issues = self.resource_specific_checks();
        if !resource_specific_issues.is_empty() {
            return Err(StackBuilderError::ResourceSpecificIssues(resource_specific_issues));
        }

        let outputs = if self.outputs.is_empty() {
            None
        } else {
            Some(self.outputs.into_iter().map(|(k, v)| (k, Output { value: v })).collect())
        };

        let metadata = self
            .resources
            .iter()
            .map(|r| (r.get_id().to_string(), r.get_resource_id().to_string()))
            .collect();

        let resources = self.resources.into_iter().map(|r| (r.get_resource_id().to_string(), r)).collect();
        Ok(Stack {
            resource_ids_to_replace: vec![],
            tags: self.tags,
            resources,
            outputs,
            metadata,
        })
    }
    
    fn resource_specific_checks(&self) -> Vec<String> {
        self.resources.iter().flat_map(|r| match r {
            Resource::Alarm(a) => {
                let mut errors = vec![];
                let props = &a.properties;
                
                if props.alarm_actions.iter().len() > 5 {
                    errors.push(format!("alarm with id {} has too many alarm actions", a.id))
                }
                if props.ok_actions.iter().len() > 5 {
                    errors.push(format!("alarm with id {} has too many ok actions", a.id))
                }
                if props.insufficient_data_actions.iter().len() > 5 {
                    errors.push(format!("alarm with id {} has too many insufficient data actions", a.id))
                }
                
                errors
            }
            _ => vec![],
        }).collect()
    }

    fn check_for_roles_with_missing_permissions(&self) -> Vec<String> {
        self.resources
            .iter()
            .filter_map(|r| match r {
                Resource::Role(r) => {
                    if !r.potentially_missing_services.is_empty() {
                        Some(format!("{}: {}", r.resource_id, r.potentially_missing_services.join(",")))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }

    fn check_for_duplicate_ids(ids: Vec<String>) -> Vec<String> {
        let results = ids.into_iter().fold((vec![], vec![]), |(mut all, mut duplicates), curr| {
            if all.contains(&curr) && !duplicates.contains(&curr) {
                duplicates.push(curr.clone());
            }
            all.push(curr);
            (all, duplicates)
        });
        results.1
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::StackBuilder;

    #[test]
    fn test_check_for_duplicate_ids() {
        let duplicates = StackBuilder::check_for_duplicate_ids(vec![
            "bucket".to_string(),
            "bucket".to_string(),
            "topic".to_string(),
            "queue".to_string(),
            "bucket".to_string(),
            "table".to_string(),
            "topic".to_string(),
        ]);

        assert_eq!(duplicates, vec!["bucket", "topic"])
    }
}
