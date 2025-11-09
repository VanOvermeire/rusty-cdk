use std::marker::PhantomData;
use crate::secretsmanager::dto::{GenerateSecretString, SecretsManagerSecret, SecretsManagerSecretProperties};
use crate::shared::Id;
use crate::stack::Resource;
use crate::wrappers::StringForSecret;

pub trait SecretsManagerSecretBuilderState {}

pub struct StartState {}
impl SecretsManagerSecretBuilderState for StartState {}

pub struct SelectedSecretTypeState {}
impl SecretsManagerSecretBuilderState for SelectedSecretTypeState {}

pub struct SecretsManagerSecretBuilder<T: SecretsManagerSecretBuilderState> {
    phantom_data: PhantomData<T>,
    id: Id,
    name: Option<String>,
    description: Option<String>,
    generate_secret_string: Option<GenerateSecretString>,
    secret_string: Option<String>,
}

impl SecretsManagerSecretBuilder<StartState> {
    pub fn new(id: &str) -> Self {
        SecretsManagerSecretBuilder {
            phantom_data: Default::default(),
            id: Id(id.to_string()),
            name: None,
            description: None,
            generate_secret_string: None,
            secret_string: None,
        }
    }

    pub fn name(self, name: StringForSecret) -> Self {
        Self {
            name: Some(name.0),
            ..self
        }
    }

    pub fn description(self, description: String) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    pub fn secret_string(self, value: String) -> SecretsManagerSecretBuilder<SelectedSecretTypeState> {
        SecretsManagerSecretBuilder {
            phantom_data: Default::default(),
            id: self.id,
            name: self.name,
            description: self.description,
            secret_string: Some(value),
            generate_secret_string: None,
        }
    }

    pub fn generate_secret_string(self, value: GenerateSecretString) -> SecretsManagerSecretBuilder<SelectedSecretTypeState> {
        SecretsManagerSecretBuilder {
            phantom_data: Default::default(),
            id: self.id,
            name: self.name,
            description: self.description,
            generate_secret_string: Some(value),
            secret_string: None,
        }
    }
}

impl SecretsManagerSecretBuilder<SelectedSecretTypeState> {
    pub fn build(self) -> SecretsManagerSecret {
        let resource_id = Resource::generate_id("SecretsManagerSecret");
        
        SecretsManagerSecret {
            id: self.id,
            resource_id,
            r#type: "AWS::SecretsManager::Secret".to_string(),
            properties: SecretsManagerSecretProperties {
                name: self.name,
                description: self.description,
                generate_secret_string: self.generate_secret_string,
                secret_string: self.secret_string,
            },
        }
    }
}

// TODO check restrictions
pub struct SecretsManagerGenerateSecretStringBuilder {
    exclude_characters: Option<String>,
    exclude_lowercase: Option<bool>,
    exclude_numbers: Option<bool>,
    exclude_punctuation: Option<bool>,
    exclude_uppercase: Option<bool>,
    generate_string_key: Option<String>,
    include_space: Option<bool>,
    password_length: Option<u32>,
    require_each_included_type: Option<bool>,
    secret_string_template: Option<String>,
}

impl SecretsManagerGenerateSecretStringBuilder {
    pub fn new() -> Self {
        Self {
            exclude_characters: None,
            exclude_lowercase: None,
            exclude_numbers: None,
            exclude_punctuation: None,
            exclude_uppercase: None,
            generate_string_key: None,
            include_space: None,
            password_length: None,
            require_each_included_type: None,
            secret_string_template: None,
        }
    }
    
    pub fn exclude_characters(self, exclude_characters: String) -> Self {
        Self {
            exclude_characters: Some(exclude_characters),
            ..self
        }
    }
    pub fn exclude_lowercase(self, exclude_lowercase: bool) -> Self {
        Self {
            exclude_lowercase: Some(exclude_lowercase),
            ..self
        }
    }

    pub fn exclude_numbers(self, exclude_numbers: bool) -> Self {
        Self {
            exclude_numbers: Some(exclude_numbers),
            ..self
        }
    }

    pub fn exclude_punctuation(self, exclude_punctuation: bool) -> Self {
        Self {
            exclude_punctuation: Some(exclude_punctuation),
            ..self
        }
    }
    
    pub fn exclude_uppercase(self, exclude_uppercase: bool) -> Self {
        Self {
            exclude_uppercase: Some(exclude_uppercase),
            ..self
        }
    }
    
    pub fn generate_string_key(self, generate_string_key: String) -> Self {
        Self {
            generate_string_key: Some(generate_string_key),
            ..self
        }
    }
    
    pub fn include_space(self, include_space: bool) -> Self {
        Self {
            include_space: Some(include_space),
            ..self
        }
    }
    
    pub fn password_length(self, password_length: u32) -> Self {
        Self {
            password_length: Some(password_length),
            ..self
        }
    }
    
    pub fn require_each_included_type(self, require_each_included_type: bool) -> Self {
        Self {
            require_each_included_type: Some(require_each_included_type),
            ..self
        }
    }
    
    pub fn secret_string_template(self, secret_string_template: String) -> Self {
        Self {
            secret_string_template: Some(secret_string_template),
            ..self
        }
    }
    
    pub fn build(self) -> GenerateSecretString {
        GenerateSecretString {
            exclude_characters: self.exclude_characters,
            exclude_lowercase: self.exclude_lowercase,
            exclude_numbers: self.exclude_numbers,
            exclude_punctuation: self.exclude_punctuation,
            exclude_uppercase: self.exclude_uppercase,
            generate_string_key: self.generate_string_key,
            include_space: self.include_space,
            password_length: self.password_length,
            require_each_included_type: self.require_each_included_type,
            secret_string_template: self.secret_string_template,
        }
    }
}