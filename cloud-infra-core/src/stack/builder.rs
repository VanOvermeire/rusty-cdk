use crate::stack::{Resource, Stack};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum StackBuilderError {
    MissingPermissionsForRole(Vec<String>),
}

impl Display for StackBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StackBuilderError::MissingPermissionsForRole(info) => {
                let gathered_info = info.join(";");
                f.write_fmt(format_args!("one or more roles seem to be missing permission to access services: `{}`?", gathered_info))
            }
        }
    }
}

impl Error for StackBuilderError {}

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
        Self { resources: vec![], tags: vec![] }
    }

    pub fn add_resource_alt<T: Into<Resource>>(&mut self, resource: T) {
        let resource = resource.into();
        self.resources.push(resource);
    }

    pub fn add_resource<T: Into<Resource>>(mut self, resource: T) -> Self {
        let resource = resource.into();
        self.resources.push(resource);
        self
    }

    pub fn add_resource_tuple<T: Into<Resource>, R: Into<Resource>>(self, resources: (T, R)) -> Self {
        self.add_resource_tuples(vec![resources])
    }

    pub fn add_resource_triple<T: Into<Resource>, R: Into<Resource>, S: Into<Resource>>(self, resources: (T, R, S)) -> Self {
        self.add_resource_triples(vec![resources])
    }

    pub fn add_resource_quadruple<T: Into<Resource>, R: Into<Resource>, S: Into<Resource>, U: Into<Resource>>(
        self,
        resources: (T, R, S, U),
    ) -> Self {
        self.add_resource_quadruples(vec![resources])
    }

    pub fn add_resources<T: Into<Resource>>(mut self, resources: Vec<T>) -> Self {
        let mut resources: Vec<_> = resources.into_iter().map(Into::into).collect();
        self.resources.append(&mut resources);
        self
    }

    pub fn add_resource_tuples<T: Into<Resource>, R: Into<Resource>>(mut self, resources: Vec<(T, R)>) -> Self {
        let mut resources: Vec<_> = resources.into_iter().flat_map(|r| [r.0.into(), r.1.into()]).collect();
        self.resources.append(&mut resources);
        self
    }

    pub fn add_resource_triples<T: Into<Resource>, R: Into<Resource>, S: Into<Resource>>(mut self, resources: Vec<(T, R, S)>) -> Self {
        let mut resources: Vec<_> = resources.into_iter().flat_map(|r| [r.0.into(), r.1.into(), r.2.into()]).collect();
        self.resources.append(&mut resources);
        self
    }

    pub fn add_resource_quadruples<T: Into<Resource>, R: Into<Resource>, S: Into<Resource>, U: Into<Resource>>(
        mut self,
        resources: Vec<(T, R, S, U)>,
    ) -> Self {
        let mut resources: Vec<_> = resources
            .into_iter()
            .flat_map(|r| [r.0.into(), r.1.into(), r.2.into(), r.3.into()])
            .collect();
        self.resources.append(&mut resources);
        self
    }

    pub fn add_tag<T: Into<String>>(mut self, key: T, value: T) -> Self {
        self.tags.push((key.into(), value.into()));
        self
    }

    pub fn build(self) -> Result<Stack, StackBuilderError> {
        let metadata = self
            .resources
            .iter()
            .map(|r| (r.get_id().to_string(), r.get_resource_id().to_string()))
            .collect();
        
        let roles_with_potentially_missing_services: Vec<_> = self.resources.iter().filter_map(|r| {
            match r {
                Resource::Role(r) => {
                    if !r.potentially_missing_services.is_empty() {
                        Some(format!("{}: {}", r.resource_id, r.potentially_missing_services.join(",")))
                    } else {
                        None
                    }
                },
                _ => None
            }
        }).collect();
        
        if !roles_with_potentially_missing_services.is_empty() {
            return Err(StackBuilderError::MissingPermissionsForRole(roles_with_potentially_missing_services))
        }

        let resources = self.resources.into_iter().map(|r| (r.get_resource_id().to_string(), r)).collect();
        Ok(Stack {
            to_replace: vec![],
            tags: self.tags,
            resources,
            metadata,
        })
    }
}
