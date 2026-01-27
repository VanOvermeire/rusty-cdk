/// Generated some methods that all resources have by passing in the resource name
#[macro_export]
macro_rules! dto_methods {
    ($name:ident) => {
        impl $name {
            #[allow(dead_code)]
            pub fn get_id(&self) -> &Id {
                &self.id
            }
        
            #[allow(dead_code)]
            pub fn get_resource_id(&self) -> &str {
                self.resource_id.as_str()
            }
        
            #[allow(dead_code)]
            pub fn get_type(&self) -> &str {
                self.r#type.as_str()
            }
        }
    };
}

#[macro_export]
macro_rules! internal_ref_struct_methods {
    () => {
        #[allow(dead_code)]
        pub(crate) fn get_resource_id(&self) -> &str {
            self.resource_id.as_str()
        }
    
        #[allow(dead_code)]
        pub fn get_ref(&self) -> Value {
            if let Some(val) = &self.ref_name {
                Value::String(val.to_string())
            } else {
                $crate::intrinsic::get_ref(self.get_resource_id())
            }
        }
        
        #[allow(dead_code)]
        pub fn get_arn(&self) -> Value {
            if let Some(val) = &self.arn_value {
                Value::String(val.to_string())
            } else {
                $crate::intrinsic::get_arn(self.get_resource_id())    
            }
        }
        
        #[allow(dead_code)]
        pub fn get_att(&self, id: &str) -> Value {
            if self.ref_name.is_some() && self.arn_value.is_some() {
                unimplemented!("get att is not supported for an imported RoleRef")
            } else {
                $crate::intrinsic::get_att(self.get_resource_id(), id)
            }
        }
    };
}

/// Generated a ref struct, which is used to reference a given resource when you need it as a dependency of some other resource.
/// To allow the other resources to depend on this one, the ref struct has methods for retrieving the Ref, ARN, and other attributes.
#[macro_export]
macro_rules! ref_struct {
    ($name:ident) => {
        #[derive(Debug,Clone)]
        pub struct $name {
            resource_id: String,
            ref_name: Option<String>,
            arn_value: Option<String>,
        }
        
        impl $name {
            #[allow(dead_code)]
            pub(crate) fn internal_new(resource_id: String) -> Self {
                Self {
                    resource_id,
                    ref_name: None,
                    arn_value: None,
                }
            }
            
            #[allow(dead_code)]
            pub fn new(resource_id: &str, ref_name: &str, arn_value: &str) -> Self {
                Self {
                    resource_id: resource_id.to_string(),
                    ref_name: Some(ref_name.to_string()),
                    arn_value: Some(arn_value.to_string()),
                }
            }
            
            crate::internal_ref_struct_methods!();
        }
    };
}

/// Generated a ref struct, which is used to reference a given resource when you need it as a dependency of some other resource.
/// To allow the other resources to depend on this one, the ref struct has methods for retrieving the Ref, ARN, and other attributes.
/// 
/// This macro also generates a method to retrieve the `id` field of the original resource, which is sometimes needed when generating custom ids based on a pre-existing id.
/// For example, SNS subscriptions have Lambdas as a subscription destination. SNS generates an additional resource for the subscription, with an id based on the Lambda id.
#[macro_export]
macro_rules! ref_struct_with_id_methods {
    ($name:ident) => {
        pub struct $name {
            id: Id,
            resource_id: String,
            ref_name: Option<String>,
            arn_value: Option<String>,
        }
        
        impl $name {
            #[allow(dead_code)]
            pub(crate) fn internal_new(id: Id, resource_id: String) -> Self {
                Self {
                    id,
                    resource_id,
                    ref_name: None,
                    arn_value: None,
                }
            }
            
            #[allow(dead_code)]
            pub fn new(id: &str, resource_id: &str, ref_name: &str, arn_value: &str) -> Self {
                Self {
                    id: Id(id.to_string()),
                    resource_id: resource_id.to_string(),
                    ref_name: Some(ref_name.to_string()),
                    arn_value: Some(arn_value.to_string()),
                }
            }

            #[allow(dead_code)]
            pub(crate) fn get_id(&self) -> &Id {
                &self.id
            }
        
            crate::internal_ref_struct_methods!();
        }
    };
}

/// Pass in the trait name, followed by names for the state structs that should implement it
/// Values separated by commas
#[macro_export]
macro_rules! type_state {
    ($state_trait:ident,$($structs:ident,)*) => {
        pub trait $state_trait {}
        $(pub struct $structs {})*
        $(impl $state_trait for $structs {})*
    };
}
