use crate::iam::{Effect, Policy};
use std::fs::read_to_string;
use std::path::Path;

const AWS_SERVICES_LIST: &str = include_str!("services_names");

pub(crate) fn find_missing_services(services: &[String], policies: &[Policy]) -> Vec<String> {
    let allow: String = Effect::Allow.into();

    let services_in_policy_actions: Vec<_> = policies
        .iter()
        .map(|p| &p.policy_document)
        .flat_map(|p| &p.statements)
        .filter(|s| s.effect == allow)
        .flat_map(|p| &p.action)
        .map(|a| a.split(":").collect::<Vec<_>>()[0])
        .collect();

    services
        .iter()
        .filter(|s| !services_in_policy_actions.contains(&s.as_str()))
        .cloned()
        .collect()
}

// async?
pub(crate) fn map_toml_dependencies_to_services(file_path: &Path) -> Vec<String> {
    let toml_as_string = read_to_string(file_path).expect("file path to be validated at compile time");
    map_string_of_tom_dependencies_to_aws_services(&toml_as_string)
}

fn map_string_of_tom_dependencies_to_aws_services(file_content: &str) -> Vec<String> {
    let services: Vec<_> = AWS_SERVICES_LIST.split("\n").collect();
    let dependencies = file_content.split("\n").skip_while(|v| !v.contains("[dependencies]"));

    dependencies
        .skip(1) // skip [dependencies]
        .take_while(|v| !v.starts_with("["))
        .filter(|v| v.contains("aws-sdk"))
        .map(|v| {
            let dependency_and_version: Vec<_> = v.split("=").collect();

            dependency_and_version[0].trim().replace("aws-sdk-", "")
        })
        .filter(|v| services.contains(&v.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::iam::permission_mapper::map_string_of_tom_dependencies_to_aws_services;

    #[test]
    fn should_find_services_mentioned_in_dependencies() {
        let services = map_string_of_tom_dependencies_to_aws_services(
            "[package]\nname = \"a-name\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ntokio = { version = \"1\", features = [\"full\"] }\nserde = { version = \"1.0.219\", features = [\"serde_derive\", \"derive\"] }\nserde_json = \"1.0.142\"\naws-config = { version = \"1.1.7\", features = [\"behavior-version-latest\"] }\naws-sdk-cloudformation = \"1.90.0\"\naws-sdk-s3 = \"1.103.0\"\n\n",
        );

        assert_eq!(services, vec!["cloudformation".to_string(), "s3".to_string()])
    }

    #[test]
    fn should_find_services_mentioned_in_dependencies_ignoring_dev_deps() {
        let services = map_string_of_tom_dependencies_to_aws_services(
            "[package]\nname = \"a-name\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ntokio = { version = \"1\", features = [\"full\"] }\nserde = { version = \"1.0.219\", features = [\"serde_derive\", \"derive\"] }\nserde_json = \"1.0.142\"\naws-config = { version = \"1.1.7\", features = [\"behavior-version-latest\"] }\naws-sdk-s3 = \"1.103.0\"\n\n[dev-dependencies]\ninsta = {  version = \"1.43.1\", features = [\"json\", \"filters\"] }\naws-sdk-cloudformation = \"1.90.0\"\nserde_json = \"1.0.142\"",
        );

        assert_eq!(services, vec!["s3".to_string()])
    }
}
