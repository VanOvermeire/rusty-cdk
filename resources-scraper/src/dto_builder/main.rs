use std::fs::{self, read_to_string};

use anyhow::{Context, Result};

// currently, this only works for _a single_ resource group
fn main() -> Result<()>{
    let resources = read_to_string("./output/raw_resources.csv")?;
    let resources = resources.split("\n").filter(|v| !v.is_empty());
    
    let mut resource_group_name = None;
    let mut output = vec!["use serde::{Deserialize, Serialize};".to_string(), "use serde_json::Value;".to_string(), "use crate::{dto_methods, ref_struct};".to_string(), "use crate::shared::Id;".to_string(), "".to_string()];
    
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

        code_to_write_for_resource.push("#[derive(Debug, Serialize, Deserialize)]".to_string());
        code_to_write_for_resource.push(format!("pub(crate) enum {} {{", type_name));
        code_to_write_for_resource.push(format!("#[serde(rename = \"{}\")]", resource_type));
        code_to_write_for_resource.push(format!("{} }}", type_name));
        
        code_to_write_for_resource.push(format!("\nref_struct!({}Ref);", struct_name));
        
        code_to_write_for_resource.push("\n#[derive(Debug, Serialize, Deserialize)]".to_string());
        code_to_write_for_resource.push(format!("pub struct {} {{", struct_name));
        code_to_write_for_resource.push("#[serde(skip)]\npub(crate) id: Id,\n#[serde(skip)]\npub(crate) resource_id: String,".to_string());
        code_to_write_for_resource.push("#[serde(rename = \"Type\")]".to_string());
        code_to_write_for_resource.push(format!("pub(crate) r#type: {},", type_name));
        code_to_write_for_resource.push("#[serde(rename = \"Properties\")]".to_string());
        code_to_write_for_resource.push(format!("pub(crate) properties: {} }}", properties_struct_name));
        
        code_to_write_for_resource.push(format!("\ndto_methods!({});", struct_name));
        
        code_to_write_for_resource.push(format!("\n#[derive(Debug, Serialize, Deserialize)]\npub struct {} {{", properties_struct_name));
        code_to_write_for_resource.push("}\n".to_string());
        // TODO write properties of Properties Struct, check whether optional, check type, add comments
        
        output.append(&mut code_to_write_for_resource);
    }
    
    // TODO create dir with right name, mod.rs
    fs::write("output/dto.rs", output.join("\n").as_bytes())?;
    
    Ok(())
}