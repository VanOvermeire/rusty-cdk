use serde_json::Value;
use crate::appconfig::dto::{Application, ApplicationProperties, ApplicationRef, ConfigurationProfile, ConfigurationProfileProperties, ConfigurationProfileRef, DeploymentStrategy, DeploymentStrategyProperties, DeploymentStrategyRef, Environment, EnvironmentProperties, EnvironmentRef, Validator};
use crate::shared::Id;
use crate::stack::{Resource, StackBuilder};
use crate::wrappers::{AppConfigName, DeploymentDurationInMinutes};

pub struct ApplicationBuilder {
    id: Id,
    name: String,
}

impl ApplicationBuilder {
    pub fn new(id: &str, name: AppConfigName) -> Self {
        Self {
            id: Id(id.to_string()),
            name: name.0,
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> ApplicationRef {
        let resource_id = Resource::generate_id("AppConfigApp");

        stack_builder.add_resource(Application {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: "AWS::AppConfig::Application".to_string(),
            properties: ApplicationProperties { name: self.name },
        });

        ApplicationRef::new(resource_id)
    }
}

// TODO macro
pub enum LocationUri {
    Hosted,
    CodePipeline(String), // codepipeline://<pipeline name>.
    SecretsManager(String), // secretsmanager://<secret name>
    S3(String) // s3://<bucket>/<objectKey>
    // SSM, AWS Systems Manager Parameter Store
}

impl From<LocationUri> for String {
    fn from(value: LocationUri) -> Self {
        match value {
            LocationUri::Hosted => "hosted".to_string(),
            LocationUri::CodePipeline(l) => l.to_string(),
            LocationUri::SecretsManager(l) => l.to_string(),
            LocationUri::S3(l) => l.to_string(),
        }
    }
}

pub enum ConfigType {
    FeatureFlags,
    Freeform,
}

impl From<ConfigType> for String {
    fn from(value: ConfigType) -> Self {
        match value {
            ConfigType::FeatureFlags => "AWS.AppConfig.FeatureFlags".to_string(),
            ConfigType::Freeform => "AWS.Freeform".to_string(),
        }
    }
}

pub enum DeletionProtectionCheck {
    AccountDefault,
    Apply,
    Bypass,
}

impl From<DeletionProtectionCheck> for String {
    fn from(value: DeletionProtectionCheck) -> Self {
        match value {
            DeletionProtectionCheck::AccountDefault => "ACCOUNT_DEFAULT".to_string(),
            DeletionProtectionCheck::Apply => "APPLY".to_string(),
            DeletionProtectionCheck::Bypass => "BYPASS".to_string(),
        }
    }
}

pub struct ConfigurationProfileBuilder {
    id: Id,
    name: String,
    application_id: Value,
    location_uri: String,
    deletion_protection_check: Option<String>,
    config_type: Option<String>,
    validators: Option<Vec<Validator>>
}

impl ConfigurationProfileBuilder {
    pub fn new(id: &str, name: AppConfigName, application: &ApplicationRef, location_uri: LocationUri) -> Self {
        Self {
            id: Id(id.to_string()),
            name: name.0,
            application_id: application.get_ref(),
            location_uri: location_uri.into(),
            deletion_protection_check: None,
            config_type: None,
            validators: None,
        }
    }

    pub fn deletion_protection_check(self, deletion_protection_check: DeletionProtectionCheck) -> Self {
        Self {
            deletion_protection_check: Some(deletion_protection_check.into()),
            ..self
        }
    }

    pub fn config_type(self, config_type: ConfigType) -> Self {
        Self {
            config_type: Some(config_type.into()),
            ..self
        }
    }

    pub fn add_validator(mut self, validator: Validator) -> Self {
        if let Some(mut validators) = self.validators {
            validators.push(validator);
            self.validators = Some(validators);
        } else {
            self.validators = Some(vec![validator]);
        }

        self
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> ConfigurationProfileRef {
        let resource_id = Resource::generate_id("ConfigurationProfile");

        stack_builder.add_resource(ConfigurationProfile {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: "AWS::AppConfig::ConfigurationProfile".to_string(),
            properties: ConfigurationProfileProperties {
                name: self.name,
                application_id: self.application_id,
                deletion_protection_check: self.deletion_protection_check,
                location_uri: self.location_uri,
                config_type: self.config_type,
                validators: self.validators,
            },
        });

        ConfigurationProfileRef::new(resource_id)
    }
}

pub struct ValidatorBuilder {
    content: String, // 0-32768
    validator_type: String, // either ARN or JSON Schema
}

impl ValidatorBuilder {
    pub fn new(content: String, validator_type: ValidatorType) -> Self {
        Self {
            content,
            validator_type: validator_type.into(),
        }
    }

    pub fn build(self) -> Validator {
        Validator {
            content: self.content,
            validator_type: self.validator_type,
        }
    }
}


pub enum ValidatorType {
    JsonSchema,
    Lambda,
}

impl From<ValidatorType> for String {
    fn from(value: ValidatorType) -> Self {
        match value {
            ValidatorType::JsonSchema => "JSON_SCHEMA".to_string(),
            ValidatorType::Lambda => "LAMBDA".to_string(),
        }
    }
}

pub enum GrowthType {
    Linear,
    Exponential,
}

impl From<GrowthType> for String {
    fn from(value: GrowthType) -> Self {
        match value {
            GrowthType::Linear => "LINEAR".to_string(),
            GrowthType::Exponential => "EXPONENTIAL".to_string()
        }
    }
}

pub struct DeploymentStrategyBuilder {
    id: Id,
    name: String,
    deployment_duration_in_minutes: u16,
    growth_factor: u16, // 0 - 100 ?
    growth_type: Option<String>,
}

impl DeploymentStrategyBuilder {
    pub fn new(id: &str, name: AppConfigName, deployment_duration_in_minutes: DeploymentDurationInMinutes, growth_factor: u16) -> Self {
        Self {
            id: Id(id.to_string()),
            name: name.0,
            deployment_duration_in_minutes: deployment_duration_in_minutes.0,
            growth_factor,
            growth_type: None,
        }
    }
    
    pub fn growth_type(self, growth_type: GrowthType) -> Self {
        Self {
            growth_type: Some(growth_type.into()),
            ..self
        }
    }
    
    pub fn build(self, stack_builder: &mut StackBuilder) -> DeploymentStrategyRef {
        let resource_id = Resource::generate_id("DeploymentStrategy");
        
        stack_builder.add_resource(DeploymentStrategy {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: "AWS::AppConfig::DeploymentStrategy".to_string(),
            properties: DeploymentStrategyProperties {
                name: self.name,
                deployment_duration_in_minutes: self.deployment_duration_in_minutes,
                growth_factor: self.growth_factor,
                growth_type: self.growth_type,
            },
        });
        
        DeploymentStrategyRef::new(resource_id)
    }
}

pub struct EnvironmentBuilder {
    id: Id,
    name: String,
    application_id: Value,
    deletion_protection_check: Option<String>
}

impl EnvironmentBuilder {
    pub fn new(id: &str, name: AppConfigName, application: &ApplicationRef) -> Self {
        Self {
            id: Id(id.to_string()),
            name: name.0,
            application_id: application.get_ref(),
            deletion_protection_check: None,
        }
    }

    pub fn deletion_protection_check(self, deletion_protection_check: DeletionProtectionCheck) -> Self {
        Self {
            deletion_protection_check: Some(deletion_protection_check.into()),
            ..self
        }
    }
    
    pub fn build(self, stack_builder: &mut StackBuilder) -> EnvironmentRef {
        let resource_id = Resource::generate_id("Environment");
        
        stack_builder.add_resource(Environment {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: "AWS::AppConfig::Environment".to_string(),
            properties: EnvironmentProperties {
                name: self.name,
                application_id: self.application_id,
                deletion_protection_check: self.deletion_protection_check,
            },
        });
        
        EnvironmentRef::new(resource_id)
    }
}
