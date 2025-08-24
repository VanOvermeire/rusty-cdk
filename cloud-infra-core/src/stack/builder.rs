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
            StackBuilderError::ReferencedIdMissingFromResources(id) => f.write_fmt(format_args!("a resource id (`{}`) was referenced by a resource, but not added to the stack - are you missing an `add_resource` call?", id))
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

    pub fn add_resource<T: Into<Resource>>(&mut self, resource: T) {
        let resource = resource.into();
        self.resources.push(resource);
    }

    pub fn build(self) -> Result<Stack, StackBuilderError> {
        let ref_ids: Vec<_> = self.resources.iter().flat_map(|r| r.get_ref_ids()).collect();
        let ids: Vec<_> = self.resources.iter().map(|r| r.get_id()).collect();

        let missing_id = ref_ids.iter().find(|r| !ids.contains(r));
        
        if let Some(missing) = missing_id {
            return Err(StackBuilderError::ReferencedIdMissingFromResources(missing.to_string()))
        }

        let resources = self.resources.into_iter().map(|r| (r.get_id().to_string(), r)).collect();
        Ok(Stack { resources })
    }
}
