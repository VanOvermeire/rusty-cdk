use std::fs::read_to_string;
use std::path::Path;

const AWS_SERVICES_LIST: &str = include_str!("services_names");

// async?
pub(crate) fn map_toml_dependencies_to_permissions(file_path: &Path) -> Vec<String> {
    let toml_as_string = read_to_string(file_path).expect("file path to be validated at compile time");
    map_string_of_tom_dependencies_to_aws_services(&toml_as_string)
}

fn map_string_of_tom_dependencies_to_aws_services(file_content: &str) -> Vec<String> {
    let services: Vec<_> = AWS_SERVICES_LIST.split("\n").collect();
    let dependencies = file_content.split("\n").skip_while(|v| !v.contains("[dependencies]"));

    dependencies
        .filter(|v| v.contains("aws-sdk"))
        .map(|v| {
            let dependency_and_version: Vec<_> = v.split("=").collect();
            let dependency_without_aws_sdk_prefix = dependency_and_version[0].trim().replace("aws-sdk-", "");
            dependency_without_aws_sdk_prefix
        })
        .filter(|v| services.contains(&v.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::iam::permission_mapper::map_string_of_tom_dependencies_to_aws_services;

    #[test]
    fn should() {
        let services = map_string_of_tom_dependencies_to_aws_services(
            "[package]\nname = \"a-name\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ntokio = { version = \"1\", features = [\"full\"] }\nserde = { version = \"1.0.219\", features = [\"serde_derive\", \"derive\"] }\nserde_json = \"1.0.142\"\naws-config = { version = \"1.1.7\", features = [\"behavior-version-latest\"] }\naws-sdk-cloudformation = \"1.90.0\"\naws-sdk-s3 = \"1.103.0\"\n\n[dev-dependencies]\ninsta = {  version = \"1.43.1\", features = [\"json\", \"filters\"] }\nserde_json = \"1.0.142\"",
        );
        
        assert_eq!(services, vec!["cloudformation".to_string(), "s3".to_string()])
    }
}
