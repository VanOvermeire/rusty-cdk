use crate::stack::{Resource, Stack};

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

    pub fn build(self) -> Stack {
        let resources = self.resources.into_iter().map(|r| (r.get_id().to_string(), r)).collect();
        Stack { resources }
    }
}
