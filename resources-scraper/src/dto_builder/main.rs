use std::fs::{self, read_to_string};

use anyhow::{Context, Result};

/// Creates DTOs for the scraped Resources 
/// Currently, this only works for _a single_ resource group
fn main() -> Result<()>{
    let resources = read_to_string("./output/raw_resources.csv")?;
    let resources = resources.split("\n").filter(|v| !v.is_empty());
    
    let mut resource_group_name = None;    
    
    let imports = r###"
        use serde::{Deserialize, Serialize};
        use serde_json::Value;
        use crate::{dto_methods, ref_struct};
        use crate::shared::Id;
    "###;
    
    let mut output = vec![imports.to_string()];
    
    for r in resources {
        let mut code_to_write_for_resource = vec![];
        
        let mut split_resource = r.split(";");
        let resource_type = split_resource.next().context("resource type to be present")?;
        let mut resource_type_parts = resource_type.split("::");
        
        if resource_group_name.is_none() {
            resource_type_parts.next();
            let group_name = resource_type_parts.next();
            resource_group_name = Some(group_name);
        }
        
        let struct_name = resource_type_parts.last().context("resource type should contain a name for the struct")?;
        let type_name = format!("{}Type", struct_name);
        let properties_struct_name = format!("{}Properties", struct_name);
        
        let code_for_type_struct = format!(r###"
            #[derive(Debug, Serialize, Deserialize)]
            pub(crate) enum {} {{
                #[serde(rename = "{}")]
                {}
            }}
        "###, type_name, resource_type, type_name);
        code_to_write_for_resource.push(code_for_type_struct);

        let code_for_main_struct = format!(r###"
            ref_struct!({}Ref);
            
            #[derive(Debug, Serialize, Deserialize)]
            pub struct {} {{
                #[serde(skip)]
                pub(crate) id: Id,
                #[serde(skip)]
                pub(crate) resource_id: String,
                #[serde(rename = "Type")]
                pub(crate) r#type: {},
                #[serde(rename = "Properties")]
                pub(crate) properties: {}
            }}

            dto_methods!({});
        "###, struct_name, struct_name, type_name, properties_struct_name, struct_name);
        code_to_write_for_resource.push(code_for_main_struct);
        
        // TODO write properties of Properties Struct, check whether optional, check type, add comments
        let properties_struct = format!(r###"
            #[derive(Debug, Serialize, Deserialize)]
            pub struct {} {{
                // TODO
            }}
        "###, properties_struct_name);
        code_to_write_for_resource.push(properties_struct);

        output.append(&mut code_to_write_for_resource);
    }
    
    // TODO create dir with right name, mod.rs
    fs::write("output/dto.rs", output.join("").as_bytes())?;
    
    Ok(())
}