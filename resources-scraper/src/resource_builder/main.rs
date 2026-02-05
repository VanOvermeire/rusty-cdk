use std::{
    fs::{self, read_to_string},
    sync::OnceLock,
};

use anyhow::{Context, Result};
use change_case::snake_case;
use regex::Regex;

static CUSTOM_PROP_TYPE_REGEX: OnceLock<Regex> = OnceLock::new();

/// Creates DTOs for the scraped Resources
/// Currently, this only works for _a single_ resource group
fn main() -> Result<()> {
    let resources = read_to_string("./output/raw_resources.csv")?;
    let mut resources = resources.split("\n").filter(|v| !v.is_empty()).peekable();

    let first_resource = resources.peek();
    let mut helper = is_helper(first_resource);

    let mut handled_resource_names = vec![];
    let mut handled_helper_names = vec![];
    let mut resource_group_name = None;

    let dto_imports = r###"use serde::{Deserialize, Serialize};
        use serde_json::Value;
        use crate::{dto_methods, ref_struct};
        use crate::shared::Id;
    "###;
    let mut dto_output = vec![dto_imports.to_string()];
    let mut builder_output = vec![];

    while let Some(r) = resources.next() {
        if helper {
            println!("found helper");
            let mut code_to_write_for_resource = vec![];

            let mut split_resource = r.split(";");

            let resource_type = split_resource.next().context("resource type to be present")?;
            let mut resource_type_parts = resource_type.split(" ");

            let prefix = resource_type_parts.next().expect("should have at least one part");
            let mut struct_name = resource_type_parts
                .next()
                .context("helper should contain a name for the struct behind a space")?
                .to_string();

            if !handled_helper_names.contains(&struct_name) {
                if handled_resource_names.contains(&struct_name) {
                    let prefix = prefix.split("::").last().context("AWS name in three parts, separated by '::'")?;
                    struct_name = format!("{}{}", prefix, &struct_name);
                }

                handled_helper_names.push(struct_name.to_string());

                let prop_infos = props_info(&mut split_resource)?;
                let props = props(&prop_infos)?;

                let helper_struct = format!(
                    r###"
                    #[derive(Debug, Serialize, Deserialize)]
                    pub struct {} {{
                        {}
                    }}
                "###,
                    struct_name,
                    props.join("\n")
                );
                code_to_write_for_resource.push(helper_struct);

                dto_output.append(&mut code_to_write_for_resource);

                let boilerplate_for_builder = builder(&struct_name, &prop_infos, true)?;
                builder_output.push(boilerplate_for_builder);
            } else {
                dto_output.push(format!(
                    "// TODO encountered a helper with name {} but one already exists - check whether they match",
                    struct_name
                ));
                builder_output.push(format!(
                    "// TODO encountered a helper with name {} but one already exists - check whether they match\n",
                    struct_name
                ));
            }
        } else {
            println!("found resource");
            let mut code_to_write_for_resource = vec![];

            let mut split_resource = r.split(";");

            let resource_type = split_resource.next().context("resource type to be present")?;
            let mut resource_type_parts = resource_type.split("::");

            if resource_group_name.is_none() {
                resource_type_parts.next();
                let group_name = resource_type_parts
                    .next()
                    .context("resource type should have three parts after splitting")?;
                resource_group_name = Some(group_name);
            }

            let struct_name = resource_type_parts
                .last()
                .context("resource type should contain a name for the struct")?;
            handled_resource_names.push(struct_name.to_string());

            let properties_struct_name = format!("{}Properties", struct_name);

            code_to_write_for_resource.append(&mut main_struct(&struct_name, &resource_type, &properties_struct_name));

            let prop_infos = props_info(&mut split_resource)?;
            let props = props(&prop_infos)?;

            let properties_struct = format!(
                r###"
                #[derive(Debug, Serialize, Deserialize)]
                pub struct {} {{
                    {}
                }}
            "###,
                properties_struct_name,
                props.join("\n")
            );
            code_to_write_for_resource.push(properties_struct);

            dto_output.append(&mut code_to_write_for_resource);

            let boilerplate_for_builder = builder(&struct_name, &prop_infos, false)?;
            builder_output.push(boilerplate_for_builder);
        }

        helper = is_helper(resources.peek());
    }

    if let Some(group_name) = resource_group_name {
        let resource_imports: Vec<_> = handled_resource_names
            .into_iter()
            .flat_map(|v| {
                vec![
                    v.clone(), format!("{}Ref", v), format!("{}Type", v), format!("{}Properties", v)
                ]
            })
            .collect();
        let helper_imports = format!("use crate::{0}::{{ {1} }};", group_name.to_lowercase(), handled_helper_names.join(", "));
        
        let mut builder_imports = vec![
            format!("use crate::{0}::{{ {1} }};", group_name.to_lowercase(), resource_imports.join(",")),
            helper_imports
        ];
        
        builder_imports.push("use crate::shared::Id;".to_string());
        builder_imports.push("use serde_json::Value;".to_string());
        builder_imports.push("use crate::stack::{Resource, StackBuilder};".to_string());
        
        let mut all_builder_info = vec![];
        all_builder_info.append(&mut builder_imports);
        all_builder_info.append(&mut builder_output);

        let output_dir = format!("output/{}", group_name.to_lowercase());

        let _ignore_if_does_not_exist = fs::remove_dir_all(&output_dir);
        let _ignore_if_already_exists = fs::create_dir(&output_dir);

        fs::write(
            &format!("{}/mod.rs", output_dir),
            "mod dto;\nmod builder;\n\npub use dto::*;\npub use builder::*;",
        )?;
        fs::write(&format!("{}/dto.rs", output_dir), dto_output.join("").as_bytes())?;
        fs::write(
            &format!("{}/builder.rs", output_dir),
            all_builder_info.join("\n").as_bytes(),
        )?;
    } else {
        println!("did not find a resource group name - not outputting")
    }

    Ok(())
}

fn main_struct(struct_name: &str, resource_type: &str, properties_struct_name: &str) -> Vec<String> {
    let type_name = format!("{}Type", struct_name);

    let code_for_type_struct = format!(
        r###"
        #[derive(Debug, Serialize, Deserialize)]
        pub(crate) enum {} {{
            #[serde(rename = "{}")]
            {}
        }}
    "###,
        type_name, resource_type, type_name
    );

    let code_for_main_struct = format!(
        r###"
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
    "###,
        struct_name, struct_name, type_name, properties_struct_name, struct_name
    );

    vec![code_for_type_struct, code_for_main_struct]
}

fn props(props: &Vec<PropInfo>) -> Result<Vec<String>> {
    let props = props
        .into_iter()
        .map(|prop| {
            let serde_info = if prop.optional {
                format!(r###"#[serde(rename = "{}", skip_serializing_if = "Option::is_none")]"###, prop.name)
            } else {
                format!(r###"#[serde(rename = "{}")]"###, prop.name)
            };

            let prop_name_and_type = if prop.optional {
                format!("pub(crate) {}: Option<{}>,", snake_case(&prop.name), prop.type_as_string)
            } else {
                format!("pub(crate) {}: {},", snake_case(&prop.name), prop.type_as_string)
            };

            format!("{}\n{}", serde_info, prop_name_and_type)
        })
        .collect();

    Ok(props)
}

struct PropInfo {
    name: String,
    type_as_string: String,
    optional: bool,
    comments: Vec<String>,
}

fn props_info(split_resource: &mut std::str::Split<&str>) -> Result<Vec<PropInfo>> {
    let custom_prop_type_regex =
        CUSTOM_PROP_TYPE_REGEX.get_or_init(|| Regex::new(r#"(?P<prefix>.*)<a href=\".+\">(?P<name>.+?)</a>"#).unwrap());

    let mut prop_info = vec![];

    while let Some(prop) = split_resource.next() {
        let mut prop_split = prop.split("===");
        let prop_name = prop_split.next().context("prop should have a name parts, before the =")?;
        let mut prop_info_split = prop_split.next().context(format!("prop should have info part, after the = (prop name {})", prop_name))?.split("###");

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
                    "String" => "String".to_string(),
                    "Integer" | "Number" => "u32".to_string(),
                    "Boolean" => "bool".to_string(),
                    "Json" => "Value".to_string(),
                    "Array of String" => "Vec<String>".to_string(),
                    _ => {
                        println!("custom type {}", prop_info);
                        let caps = custom_prop_type_regex
                            .captures(&prop_info)
                            .context("failed to capture custom type information")?;
                        let name = caps["name"].to_string();

                        if caps["prefix"].is_empty() {
                            name
                        } else if caps["prefix"].trim() == "Array of" {
                            format!("Vec<{}>", &caps["name"])
                        } else if caps["prefix"].trim() == "Object of" {
                            format!("HashMap<{0}, {0}>", &caps["name"])
                        } else {
                            panic!("encountered unknown prefix {}", &caps["prefix"]);
                        }
                    }
                };
            } else {
                comments.push(prop_info.to_string());
            }
        }
        prop_info.push(PropInfo {
            name: prop_name.to_string(),
            type_as_string: type_info,
            optional,
            comments,
        });
    }

    Ok(prop_info)
}

fn builder(struct_name: &str, props: &Vec<PropInfo>, helper: bool) -> Result<String> {
    let mut struct_fields_definition: Vec<_> = props
        .iter()
        .map(|v| {
            if v.optional {
                format!("{}: Option<{}>, // {}", snake_case(&v.name), v.type_as_string, v.comments.join(", "))
            } else {
                format!("{}: {}, // {}", snake_case(&v.name), v.type_as_string, v.comments.join(", "))
            }
        })
        .collect();
    if !helper {
        struct_fields_definition.insert(0, "id: Id,".to_string());
    }
    let struct_fields_definition = struct_fields_definition.join("\n");

    let mut struct_fields_init: Vec<_> = props
        .iter()
        .map(|v| {
            if v.optional {
                format!("{}: None", snake_case(&v.name))
            } else {
                snake_case(&v.name)
            }
        })
        .collect();
    if !helper {
        struct_fields_init.insert(0, "id: Id(id.to_string())".to_string());
    }
    let struct_fields_init = struct_fields_init.join(",\n");

    let mut struct_constructor_args: Vec<_> = props
        .iter()
        .filter_map(|v| {
            if !v.optional {
                Some(format!("{}: {}", snake_case(&v.name), v.type_as_string))
            } else {
                None
            }
        })
        .collect();
    if !helper {
        struct_constructor_args.insert(0, "id: &str".to_string());
    }
    let struct_constructor_args = struct_constructor_args.join(",");

    let optional_prop_builders = props
        .iter()
        .filter(|v| v.optional)
        .map(|prop| {
            format!(
                r###"
            pub fn {0}(self, {0}: {1}) -> Self {{
                Self {{
                    {0}: Some({0}),
                    ..self
                }}
            }}
            "###,
                snake_case(&prop.name),
                prop.type_as_string
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let build_method_props = props
        .iter()
        .map(|v| format!("{0}: self.{0}", snake_case(&v.name)))
        .collect::<Vec<_>>()
        .join(",\n");
    let build_method = if helper {
        format!(
            r###"
            pub fn build(self) -> {0} {{
                {0} {{
                    {1}
                }}
            }}
        "###,
            struct_name, build_method_props
        )
    } else {
        format!(
            r###"
            pub fn build(self, stack_builder: &mut StackBuilder) -> {0}Ref {{
                let resource_id = Resource::generate_id("{0}");           
                
                let resource = {0} {{
                     id: self.id,
                     resource_id: resource_id.clone(),
                     r#type: {0}Type::{0}Type,
                     properties: {0}Properties {{
                         {1}
                     }},
                }};
                // stack_builder.add_resource(resource); // TODO add to Resource enum to activate!
                
                {0}Ref::internal_new(resource_id)
            }}
        "###,
            struct_name, build_method_props
        )
    };

    let builder = format!(
        r###"
        pub struct {0}Builder {{
            {1}
        }}
        
        impl {0}Builder {{
            pub fn new({2}) -> Self {{
                Self {{
                    {3}
                }}
            }}
            
            {4}
            
            {5}
        }}
    "###,
        struct_name, struct_fields_definition, struct_constructor_args, struct_fields_init, optional_prop_builders, build_method,
    );

    Ok(builder)
}

fn is_helper(resource: Option<&&str>) -> bool {
    resource
        .map(|v| {
            let value = v.split(";").next().unwrap_or("");
            value.contains("::") && value.contains(" ")
        })
        .unwrap_or(false)
}
