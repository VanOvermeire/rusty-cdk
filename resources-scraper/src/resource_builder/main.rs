use std::fs::{self, read_to_string};

use anyhow::{Context, Result};
use change_case::snake_case;

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

    let mut dto_output = vec![imports.to_string()];
    let mut builder_output = vec!["use crate::shared::Id;".to_string()];
    
    for r in resources {
        let mut code_to_write_for_resource = vec![];
        
        let mut split_resource = r.split(";");
        let resource_type = split_resource.next().context("resource type to be present")?;
        let mut resource_type_parts = resource_type.split("::");
        
        if resource_group_name.is_none() {
            resource_type_parts.next();
            let group_name = resource_type_parts.next().context("resource type should have three parts after splitting")?;
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
        
        let boilerplate_for_builder = format!(r###"
            pub struct {0}Builder {{
                id: Id,
            }}
            
            impl {0}Builder {{
                pub fn new(id: Id) -> Self {{
                    Self {{
                        id,
                    }}
                }}
            }}
        "###, struct_name);
        builder_output.push(boilerplate_for_builder);
        
        let mut props = vec![];
        
        while let Some(prop) = split_resource.next() {
            let mut prop_split = prop.split("===");
            let prop_name = prop_split.next().context("prop should have a name parts, before the =")?;
            let mut prop_info_split = prop_split.next().context("prop should have info part, after the =")?.split("###");
            
            let mut optional = false;
            let mut type_info = "".to_string();
            let mut comments = vec![];
            
            while let Some(prop_info) = prop_info_split.next() {
                let prop_info = prop_info.trim();
                
                if prop_info == "Required: No" {
                    optional = true;
                } else if prop_info.starts_with("Type: ") {
                    let prop_info = prop_info.replace("Type: ", "");
                    type_info = match prop_info.as_str() {
                        "String" => "String",
                        "Integer" => "u32",
                        "Boolean" => "bool",
                        "Array of String" => "Vec<String>",
                        _ => { 
                            println!("could not find type for {}", prop_info);
                            "TODO"
                        }
                    }.to_string();
                } else {
                    comments.push(prop_info);
                }
            }
            
            let serde_info = if optional {
                format!(r###"#[serde(rename = "{}", skip_serializing_if = "Option::is_none")]"###, prop_name)
            } else {
                format!(r###"#[serde(rename = "{}")]"###, prop_name)
            };
            
            let prop_name_and_type = if optional {
                format!("pub(crate) {}: Option<{}>,", snake_case(prop_name), type_info)
            } else {
                format!("pub(crate) {}: {},", snake_case(prop_name), type_info)
            };

            props.push(format!("{}\n{} // {}", serde_info, prop_name_and_type, comments.join(", ")));
        }
        
        let properties_struct = format!(r###"
            #[derive(Debug, Serialize, Deserialize)]
            pub struct {} {{
                {}
            }}
        "###, properties_struct_name, props.join("\n"));
        code_to_write_for_resource.push(properties_struct);

        dto_output.append(&mut code_to_write_for_resource);
    }
    
    let resource_group_name = resource_group_name.unwrap().to_lowercase();
    let output_dir = format!("output/{}", resource_group_name);
    
    let _ignore_if_does_not_exist = fs::remove_dir_all(&output_dir);
    fs::create_dir(&output_dir)?; // will fail if directory already exists
    fs::write(&format!("{}/mod.rs", output_dir), "mod dto;\n\npub use dto::*;")?;
    fs::write(&format!("{}/dto.rs", output_dir), dto_output.join("").as_bytes())?;
    fs::write(&format!("{}/builder.rs", output_dir), builder_output.join("\n").as_bytes())?;
    
    Ok(())
}