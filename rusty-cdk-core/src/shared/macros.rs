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
        }
        
        impl $name {
            #[allow(dead_code)]
            pub fn new(resource_id: String) -> Self {
                Self {
                    resource_id,
                }
            }
        
            #[allow(dead_code)]
            pub fn get_resource_id(&self) -> &str {
                self.resource_id.as_str()
            }
        
            #[allow(dead_code)]
            pub fn get_ref(&self) -> Value {
                $crate::intrinsic::get_ref(self.get_resource_id())
            }
            
            #[allow(dead_code)]
            pub fn get_arn(&self) -> Value {
                $crate::intrinsic::get_arn(self.get_resource_id())
            }
            
            #[allow(dead_code)]
            pub fn get_att(&self, id: &str) -> Value {
                $crate::intrinsic::get_att(self.get_resource_id(), id)
            }
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
        }
        
        impl $name {
            #[allow(dead_code)]
            pub fn new(id: Id, resource_id: String) -> Self {
                Self {
                    id,
                    resource_id,
                }
            }

            #[allow(dead_code)]
            pub fn get_id(&self) -> &Id {
                &self.id
            }
        
            #[allow(dead_code)]
            pub fn get_resource_id(&self) -> &str {
                self.resource_id.as_str()
            }
        
            #[allow(dead_code)]
            pub fn get_ref(&self) -> Value {
                $crate::intrinsic::get_ref(self.get_resource_id())
            }
            
            #[allow(dead_code)]
            pub fn get_arn(&self) -> Value {
                $crate::intrinsic::get_arn(self.get_resource_id())
            }
            
            #[allow(dead_code)]
            pub fn get_att(&self, id: &str) -> Value {
                $crate::intrinsic::get_att(self.get_resource_id(), id)
            }
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
