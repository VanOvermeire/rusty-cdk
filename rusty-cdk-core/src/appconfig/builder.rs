use serde_json::Value;
use crate::appconfig::{Application, ApplicationProperties, ApplicationRef, ConfigurationProfile, ConfigurationProfileProperties, ConfigurationProfileRef, DeploymentStrategy, DeploymentStrategyProperties, DeploymentStrategyRef, Environment, EnvironmentProperties, EnvironmentRef, Validator};
use crate::shared::Id;
use crate::stack::{Resource, StackBuilder};
use crate::wrappers::{AppConfigName, DeploymentDurationInMinutes, GrowthFactor, LocationUri};

/// Builder for AWS AppConfig applications.
///
/// # Example
///
/// ```rust
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::appconfig::ApplicationBuilder;
/// use rusty_cdk_core::wrappers::*;
/// use rusty_cdk_macros::app_config_name;
///
/// let mut stack_builder = StackBuilder::new();
///
/// let app = ApplicationBuilder::new("my-app", app_config_name!("MyApplication"))
///     .build(&mut stack_builder);
/// ```
pub struct ApplicationBuilder {
    id: Id,
    name: String,
}

impl ApplicationBuilder {
    /// Creates a new AppConfig application builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the application
    /// * `name` - Name of the AppConfig application
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

// pub enum LocationUri {
//     Hosted,
//     CodePipeline(String), // codepipeline://<pipeline name>.
//     SecretsManager(String), // secretsmanager://<secret name>
//     S3(String) // s3://<bucket>/<objectKey>
//     // SSM, AWS Systems Manager Parameter Store
// }
// 
// impl From<LocationUri> for String {
//     fn from(value: LocationUri) -> Self {
//         match value {
//             LocationUri::Hosted => "hosted".to_string(),
//             LocationUri::CodePipeline(l) => l.to_string(),
//             LocationUri::SecretsManager(l) => l.to_string(),
//             LocationUri::S3(l) => l.to_string(),
//         }
//     }
// }

pub enum ConfigType {
    FeatureFlags,
    Freeform,
}

// TODO might be more idiomatic to implement display
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

/// Builder for AWS AppConfig configuration profiles.
///
/// # Example
///
/// ```rust,no_run
/// use rusty_cdk_core::stack::StackBuilder;
/// use rusty_cdk_core::appconfig::{ApplicationBuilder, ConfigurationProfileBuilder};
/// use rusty_cdk_core::wrappers::*;
/// use rusty_cdk_macros::{app_config_name, location_uri};
///
/// let mut stack_builder = StackBuilder::new();
///
/// let app = unimplemented!("create an application");
/// let location_uri = location_uri!("hosted");
///
/// let profile = ConfigurationProfileBuilder::new(
///     "my-profile",
///     app_config_name!("MyProfile"),
///     &app,
///     location_uri,
/// )
/// .build(&mut stack_builder);
/// ```
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
    /// Creates a new AppConfig configuration profile builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the configuration profile
    /// * `name` - Name of the configuration profile
    /// * `application` - Reference to the parent AppConfig application
    /// * `location_uri` - Location where the configuration is stored
    pub fn new(id: &str, name: AppConfigName, application: &ApplicationRef, location_uri: LocationUri) -> Self {
        Self {
            id: Id(id.to_string()),
            name: name.0,
            application_id: application.get_ref(),
            location_uri: location_uri.0,
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

/// Builder for configuration profile validators.
pub struct ValidatorBuilder {
    content: String,
    validator_type: String,
}

impl ValidatorBuilder {
    // could validate better with a macro, but for JSON that would require passing in the entire schema in the macro...
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

pub enum ReplicateTo {
    None,
    SsmDocument,
}

impl From<ReplicateTo> for String {
    fn from(value: ReplicateTo) -> Self {
        match value {
            ReplicateTo::None => "NONE".to_string(),
            ReplicateTo::SsmDocument => "SSM_DOCUMENT".to_string(),
        }
    }
}

/// Builder for AWS AppConfig deployment strategies.
pub struct DeploymentStrategyBuilder {
    id: Id,
    name: String,
    deployment_duration_in_minutes: u16,
    growth_factor: u8,
    growth_type: Option<String>,
    replicate_to: String,
}

impl DeploymentStrategyBuilder {
    /// Creates a new AppConfig deployment strategy builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the deployment strategy
    /// * `name` - Name of the deployment strategy
    /// * `deployment_duration_in_minutes` - Time to deploy the configuration
    /// * `growth_factor` - Percentage of targets to receive the deployment during each interval
    /// * `replicate_to` - Where to replicate the configuration
    pub fn new(id: &str, name: AppConfigName, deployment_duration_in_minutes: DeploymentDurationInMinutes, growth_factor: GrowthFactor, replicate_to: ReplicateTo) -> Self {
        Self {
            id: Id(id.to_string()),
            name: name.0,
            deployment_duration_in_minutes: deployment_duration_in_minutes.0,
            growth_factor: growth_factor.0,
            growth_type: None,
            replicate_to: replicate_to.into(),
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
                replicate_to: self.replicate_to,
                growth_type: self.growth_type,
            },
        });
        
        DeploymentStrategyRef::new(resource_id)
    }
}

/// Builder for AWS AppConfig environments.
pub struct EnvironmentBuilder {
    id: Id,
    name: String,
    application_id: Value,
    deletion_protection_check: Option<String>
}

impl EnvironmentBuilder {
    /// Creates a new AppConfig environment builder.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the environment
    /// * `name` - Name of the environment
    /// * `application` - Reference to the parent AppConfig application
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
