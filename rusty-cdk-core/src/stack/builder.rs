use crate::stack::{Resource, Stack};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::shared::Id;

#[derive(Debug)]
pub enum StackBuilderError {
    MissingPermissionsForRole(Vec<String>),
    DuplicateIds(Vec<String>),
    DuplicateResourceIds(Vec<String>),
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
            StackBuilderError::DuplicateResourceIds(info) => {
                let gathered_info = info.join(";");
                f.write_fmt(format_args!(
                    "duplicate resource ids were detected (`{}`), rerunning this command should generate new ones",
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
    
    pub(crate) fn get_resource(&mut self, id: &Id) -> Option<&mut Resource> {
        self.resources.iter_mut()
            .find(|v| &v.get_id() == id)
    }

    /// Builds the stack and validates all resources.
    ///
    /// Might return an error if any IAM roles are missing permissions for AWS services they need to access, based on Cargo.toml dependencies.
    pub fn build(self) -> Result<Stack, StackBuilderError> {
        let (ids, resource_ids) = self.resources
            .iter()
            .map(|r| (r.get_id().to_string(), r.get_resource_id().to_string()))
            .collect::<(Vec<_>, Vec<_>)>();
        
        let duplicate_ids = Self::check_for_duplicate_ids(ids);
        let resource_ids = Self::check_for_duplicate_ids(resource_ids);
        
        if !duplicate_ids.is_empty() {
            return Err(StackBuilderError::DuplicateIds(
                duplicate_ids,
            ));
        }
        if !resource_ids.is_empty() {
            return Err(StackBuilderError::DuplicateResourceIds(
                resource_ids,
            ));
        }
        
        let roles_with_potentially_missing_services: Vec<_> = self.check_for_roles_with_missing_permissions();

        if !roles_with_potentially_missing_services.is_empty() {
            return Err(StackBuilderError::MissingPermissionsForRole(
                roles_with_potentially_missing_services,
            ));
        }
        

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
            metadata,
        })
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
            return (all, duplicates);
        });
        results.1
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::StackBuilder;

    #[test]
    fn test_check_for_duplicate_ids() {
        let duplicates = StackBuilder::check_for_duplicate_ids(vec!["bucket".to_string(), "bucket".to_string(), "topic".to_string(), "queue".to_string(), "bucket".to_string(), "table".to_string(), "topic".to_string()]);
        
        assert_eq!(duplicates, vec!["bucket", "topic"])
    }
}
