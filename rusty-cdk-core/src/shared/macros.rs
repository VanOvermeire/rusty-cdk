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

#[macro_export]
macro_rules! ref_struct {
    ($name:ident) => {
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

// most of the time, resource id is enough
// but lambda functions, for example, needs the `id` field for creating some custom ids used by API Gateway and subscriptions
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

#[macro_export]
macro_rules! type_state {
    ($state_trait:ident,$($structs:ident,)*) => {
        pub trait $state_trait {}
        $(pub struct $structs {})*
        $(impl $state_trait for $structs {})*
    };
}
