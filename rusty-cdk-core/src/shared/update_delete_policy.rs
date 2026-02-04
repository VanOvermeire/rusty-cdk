use serde::{Deserialize, Serialize};

pub enum UpdateReplacePolicy {
    /// `Delete` is the default, deleting the resource (if possible)
    Delete,
    /// `Snapshot` deletes it after creating a snapshot
    Snapshot,
    /// `Retain` keeps the resource
    Retain,
}

impl From<UpdateReplacePolicy> for String {
    fn from(value: UpdateReplacePolicy) -> String {
        match value {
            UpdateReplacePolicy::Delete => "Delete".to_string(),
            UpdateReplacePolicy::Snapshot => "Snapshot".to_string(),
            UpdateReplacePolicy::Retain => "Retain".to_string(),
        }
    }
}

/// Determines what happens with an existing resource when it is deleted
pub enum DeletionPolicy {
    /// `Delete` deletes the resource (default)
    Delete,
    /// `Snapshot` deletes it after creating a snapshot (*if snapshots are possible*)
    Snapshot,
    /// `Retain` keeps the resource
    Retain,
    /// `RetainExceptOnCreate` keeps the resource, except when it was just created during this stack operation
    RetainExceptOnCreate,
}

impl From<DeletionPolicy> for String {
    fn from(value: DeletionPolicy) -> String {
        match value {
            DeletionPolicy::Delete => "Delete".to_string(),
            DeletionPolicy::Snapshot => "Snapshot".to_string(),
            DeletionPolicy::Retain => "Retain".to_string(),
            DeletionPolicy::RetainExceptOnCreate => "RetainExceptOnCreate".to_string(),
        }
    }
}

impl From<&String> for DeletionPolicy {
    fn from(value: &String) -> DeletionPolicy {
        match value.as_str() {
            "Delete" => DeletionPolicy::Delete,
            "Snapshot" => DeletionPolicy::Snapshot,
            "Retain" => DeletionPolicy::Retain,
            "RetainExceptOnCreate" => DeletionPolicy::RetainExceptOnCreate,
            _ => unreachable!("all deletion policy options to be covered"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeletePolicyDTO {
    #[serde(rename = "DeletionPolicy", skip_serializing_if = "Option::is_none")]
    pub(crate) deletion_policy: Option<String>,
    #[serde(rename = "UpdateReplacePolicy", skip_serializing_if = "Option::is_none")]
    pub(crate) update_replace_policy: Option<String>,
}
