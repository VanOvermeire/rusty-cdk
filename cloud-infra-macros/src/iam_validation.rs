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
                    if permission.contains("*") {
                        self.handle_wildcards(service, permission, valid_permissions)   
                    } else if valid_permissions.contains(&permission) {
                        ValidationResponse::Valid
                    } else {
                        ValidationResponse::Invalid(format!("{} LOL not exist for {}", permission, service))
                    }
                } else {
                    ValidationResponse::Invalid(format!("unknown AWS service name: {}", service))
                }
            },
            _ => ValidationResponse::Invalid("expected a service and its permission separated by :".to_string())
        }
    }
    
    // Does not cover the (rare) case of wildcards in the middle of permissions.
    // To handle that without writing too much code, I'd need to add regex. 
    // For now, accept that a user who adds a lot of wildcards knows what he or she's doing :)
    fn handle_wildcards(&self, service: &str, permission: &str, valid_permissions: &Vec<&str>) -> ValidationResponse {
        if permission == "*" {
            ValidationResponse::Valid
        } else {
            let permission_without_wildcard = permission.replace("*", "");
            
            if permission.starts_with("*") && permission.ends_with("*") {
                let part_of_valid_permission = valid_permissions.iter().any(|v| v.contains(&permission_without_wildcard));
                
                if part_of_valid_permission {
                    ValidationResponse::Valid
                } else {
                    ValidationResponse::Invalid(format!("no valid permission contains {} (for service: {})", permission_without_wildcard, service))
                }
            } else if permission.starts_with("*") && !valid_permissions.iter().any(|v| v.ends_with(&permission_without_wildcard)) {
                ValidationResponse::Invalid(format!("no valid permission ends with {} (for service: {})", permission_without_wildcard, service))
            } else if permission.ends_with("*") && !valid_permissions.iter().any(|v| v.starts_with(&permission_without_wildcard)) {
                ValidationResponse::Invalid(format!("no valid permission starts with {} (for service: {})", permission_without_wildcard, service))
            } else {
                ValidationResponse::Valid
            }
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
    fn test_validates_valid_action_string_with_wildcard_only() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("dynamodb:*");

        assert_eq!(result, ValidationResponse::Valid);
    }

    #[test]
    fn test_validates_valid_action_string_with_wildcard_at_end() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("dynamodb:Batch*");

        assert_eq!(result, ValidationResponse::Valid);
    }

    #[test]
    fn test_validates_valid_action_string_with_wildcard_at_start() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("dynamodb:*GetItem");

        assert_eq!(result, ValidationResponse::Valid);
    }

    #[test]
    fn test_validates_valid_action_string_with_multiple_wildcards() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("dynamodb:*Get*");

        assert_eq!(result, ValidationResponse::Valid);
    }

    #[test]
    fn test_validates_invalid_action_string() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("dynamodb:Fake");

        assert_ne!(result, ValidationResponse::Valid);
    }

    #[test]
    fn test_validates_unknown_service() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("unknown:BatchWriteItem");

        assert_ne!(result, ValidationResponse::Valid);
    }

    #[test]
    fn test_validates_invalid_action_string_with_wildcard() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("dynamodb:Fake*");

        assert_ne!(result, ValidationResponse::Valid);
    }

    #[test]
    fn test_validates_invalid_action_string_with_multiple_wildcards() {
        let list = "dynamodb;BatchGetItem,Scan\ndax;BatchWriteItem";
        let validator = PermissionValidator {
            services_with_permissions: create_permissions_map_for(list),
        };

        let result = validator.is_valid_action("dynamodb:*Fake*");

        assert_ne!(result, ValidationResponse::Valid);
    }
}