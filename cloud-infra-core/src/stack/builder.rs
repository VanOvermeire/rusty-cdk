use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::stack::{Resource, Stack};

#[derive(Debug)]
pub enum StackBuilderError {
    ReferencedIdMissingFromResources(String)
}

impl Display for StackBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StackBuilderError::ReferencedIdMissingFromResources(id) => {
                // works for some ids, for event source mappings the proposed resource will still be confusing
                let id_without_suffix: String = id.chars().take_while(|c| !c.is_ascii_digit()).collect();
                f.write_fmt(format_args!("a resource id (`{}`) was referenced by a resource, but not added to the stack - are you missing an `add_resource` call for a `{}`?", id, id_without_suffix))
            }
        }
    }
}

impl Error for StackBuilderError {}

pub struct StackBuilder {
    resources: Vec<Resource>,
}

impl StackBuilder {
    pub fn new() -> Self {
        Self {
            resources: vec![]
        }
    }

    pub fn add_resource<T: Into<Resource>>(mut self, resource: T) -> Self {
        let resource = resource.into();
        self.resources.push(resource);
        self
    }
    
    pub fn add_resource_tuple<T: Into<Resource>, R: Into<Resource>>(mut self, resources: (T, R)) -> Self {
        self.resources.push(resources.0.into());
        self.resources.push(resources.1.into());
        self
    }
    
    pub fn add_resource_triple<T: Into<Resource>, R: Into<Resource>, S: Into<Resource>>(mut self, resources: (T, R, S)) -> Self {
        self.resources.push(resources.0.into());
        self.resources.push(resources.1.into());
        self.resources.push(resources.2.into());
        self
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

    pub fn build(self) -> Result<Stack, StackBuilderError> {
        let ref_ids: Vec<_> = self.resources.iter().flat_map(|r| r.get_refenced_ids()).collect();
        let ids: Vec<_> = self.resources.iter().map(|r| r.get_resource_id()).collect();

        let missing_id = ref_ids.iter().find(|r| !ids.contains(r));
        
        if let Some(missing) = missing_id {
            return Err(StackBuilderError::ReferencedIdMissingFromResources(missing.to_string()))
        }

        let resources = self.resources.into_iter().map(|r| (r.get_resource_id().to_string(), r)).collect();
        Ok(Stack { resources })
    }
}
