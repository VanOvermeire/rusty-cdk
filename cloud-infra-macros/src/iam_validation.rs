use std::collections::HashMap;

const IAM_PERMISSIONS_LIST: &str = include_str!("permissions.csv");

#[derive(Debug, PartialEq)]
pub(crate) enum ValidationResponse {
    Valid,
    Invalid(String),
}

pub(crate) struct PermissionValidator {
    services_with_permissions: HashMap<&'static str, Vec<&'static str>>,
}

impl PermissionValidator {
    pub(crate) fn new() -> Self {
        Self {
            services_with_permissions: create_permissions_map_for(IAM_PERMISSIONS_LIST)
        }
    }

    pub(crate) fn is_valid_action(&self, action: &str) -> ValidationResponse {
        // TODO is whitespace before/after an action valid?
        // TODO is wildcard valid anywhere? if only at end, throw error. if valid anywhere, change code
        let mut service_and_permission: Vec<_> = action.split(':').collect();
        let permission = service_and_permission.pop();
        let service = service_and_permission.pop();
        
        match (service, permission) {
            (Some(service), Some(permission)) => {
                let valid_permissions = self.services_with_permissions.get(service);
                
                if let Some(valid_permissions) = valid_permissions {
                    if permission.ends_with("*") {
                        let permission_without_wildcard = permission.replace("*", "");
                        
                        if valid_permissions.iter().any(|v| v.starts_with(&permission_without_wildcard)) {
                            ValidationResponse::Valid
                        } else {
                            ValidationResponse::Invalid(format!("{} does not exist for {}", permission, service))
                        }
                    } else if valid_permissions.contains(&permission) {
                        ValidationResponse::Valid
                    } else {
                        ValidationResponse::Invalid(format!("{} does not exist for {}", permission, service))
                    }
                } else {
                    ValidationResponse::Invalid(format!("unknown AWS service name: {}", service))
                }
            },
            _ => ValidationResponse::Invalid("expected a service and its permission separated by :".to_string())
        }
    }
}

fn create_permissions_map_for(list: &'static str) -> HashMap<&'static str, Vec<&'static str>> {
    let services_and_permissions: Vec<(&str, Vec<&str>)> = list
        .split('\n')
        .filter(|m| !m.is_empty())
        .map(|m| {
            let mut service_and_permissions: Vec<_> = m.split(';').collect();
            let permissions = service_and_permissions.pop()
                .expect("required props to be the second element")
                .split(',')
                .collect();
            let service_name = service_and_permissions.pop().expect("service to be the first element");
            (service_name, permissions)
        })
        .collect();
    services_and_permissions.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_permissions_map_creates_map_with_entries_by_method_name_containing_hashmaps_by_service_key() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";

        let permissions = create_permissions_map_for(list);

        assert_eq!(permissions.keys().count(), 2);
        let dynamo = permissions.get("dynamodb").unwrap();
        let dax = permissions.get("dax").unwrap();
        assert_eq!(dynamo, &vec!["BatchGetItem", "Scan"]);
        assert_eq!(dax, &vec!["BatchWriteItem"]);
    }

    #[test]
    fn test_validates_valid_action_string() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("dax:BatchWriteItem");

        assert_eq!(result, ValidationResponse::Valid);
    }

    #[test]
    fn test_validates_invalid_action_string() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("dynamodb:Fake");

        match result {
            ValidationResponse::Valid => unreachable!("output should be invalid"),
            ValidationResponse::Invalid(_) => {}
        }
    }

    #[test]
    fn test_validates_unknown_service() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("unknown:BatchWriteItem");

        match result {
            ValidationResponse::Valid => unreachable!("output should be invalid"),
            ValidationResponse::Invalid(_) => {}
        }
    }

    #[test]
    fn test_validates_valid_action_string_with_wildcard() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("dynamodb:Batch*");

        assert_eq!(result, ValidationResponse::Valid);
    }

    #[test]
    fn test_validates_invalid_action_string_with_wildcard() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("dynamodb:Fake*");

        match result {
            ValidationResponse::Valid => unreachable!("output should be 'invalid'"),
            ValidationResponse::Invalid(_) => {}
        }
    }
}