#[macro_export]
macro_rules! ref_struct {
    ($name:ident) => {
        pub struct $name {
            resource_id: String,
        }
        
        impl $name {
            pub fn new(resource_id: String) -> Self {
                Self {
                    resource_id,
                }
            }
        
            pub fn get_resource_id(&self) -> &str {
                self.resource_id.as_str()
            }
        
            pub fn get_ref(&self) -> Value {
                crate::intrinsic_functions::get_ref(self.get_resource_id())
            }
            
            pub fn get_arn(&self) -> Value {
                crate::intrinsic_functions::get_arn(self.get_resource_id())
            }
            
            pub fn get_att(&self, id: &str) -> Value {
                crate::intrinsic_functions::get_att(self.get_resource_id(), id)
            }
        }
    };
}
