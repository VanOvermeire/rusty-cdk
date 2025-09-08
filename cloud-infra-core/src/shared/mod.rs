use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub mod http;

#[derive(Debug, Clone)]
pub struct Id(pub String);

impl Deref for Id {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Id {
    pub fn generate_id(id: &Id, suffix: &str) -> Id {
        Id(format!("{}{}", id.0, suffix))
    }
    
    pub fn combine_ids(first: &Id, second: &Id) -> Id {
        Id(format!("{}{}", first.0, second.0))
    }
}