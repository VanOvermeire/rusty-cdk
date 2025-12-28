use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub mod http;
pub mod macros;

/// `Id` is a unique identifier for a resource within a stack, chosen by the user (if it's a root resource), or by the library itself.
/// E.g., when a user adds a bucket, he/she will pick the id of that bucket.
/// But when we automatically generate additional resources for that bucket (a policy, for example), the library chooses those additional ids, often by adding a suffix.
/// 
/// These ids differ from 'resource ids', which are the names given to resources in the CloudFormation template, randomly generated during synth.
///
/// Ids make sure we do not replace existing resources when we're dealing with an existing stack.
/// Because they are unique and should not change, we can link a chosen id with an existing resource id.
/// 
/// A user should not change an id if he/she does not want to replace an existing resource.
/// This behavior is similar to that of the AWS CDK
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
    
    pub fn combine_with_resource_id(first: &Id, second: &str) -> Id {
        Id(format!("{}{}", first.0, second))
    }
}