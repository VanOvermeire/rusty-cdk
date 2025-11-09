use std::marker::PhantomData;
use serde_json::Value;
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
    #[must_use]
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



pub trait SecretsManagerGenerateSecretStringBuilderState {}

pub struct GenerateStringStartState {}
impl SecretsManagerGenerateSecretStringBuilderState for GenerateStringStartState {}

pub struct GenerateStringKeyState {}
impl SecretsManagerGenerateSecretStringBuilderState for GenerateStringKeyState {}

pub struct SecretStringTemplateState {}
impl SecretsManagerGenerateSecretStringBuilderState for SecretStringTemplateState {}

pub struct SecretsManagerGenerateSecretStringBuilder<T: SecretsManagerGenerateSecretStringBuilderState> {
    phantom_data: PhantomData<T>,
    generate_string_key: Option<String>,
    secret_string_template: Option<String>,
    exclude_characters: Option<Vec<char>>,
    exclude_lowercase: Option<bool>,
    exclude_numbers: Option<bool>,
    exclude_punctuation: Option<bool>,
    exclude_uppercase: Option<bool>,
    include_space: Option<bool>,
    password_length: Option<u32>,
    require_each_included_type: Option<bool>,
}

impl<T: SecretsManagerGenerateSecretStringBuilderState> SecretsManagerGenerateSecretStringBuilder<T> {
    fn build_internal(self) -> GenerateSecretString {
        GenerateSecretString {
            exclude_characters: self.exclude_characters.map(|v| v.into_iter().collect()),
            exclude_lowercase: self.exclude_lowercase,
            exclude_numbers: self.exclude_numbers,
            exclude_punctuation: self.exclude_punctuation,
            exclude_uppercase: self.exclude_uppercase,
            include_space: self.include_space,
            password_length: self.password_length,
            require_each_included_type: self.require_each_included_type,
            generate_string_key: self.generate_string_key,
            secret_string_template: self.secret_string_template,
        }
    }
}

impl SecretsManagerGenerateSecretStringBuilder<GenerateStringStartState> {
    pub fn new() -> Self {
        Self {
            phantom_data: Default::default(),
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
    
    pub fn exclude_characters(self, exclude_characters: Vec<char>) -> Self {
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

    pub fn generate_string_key(self, generate_string_key: String) -> SecretsManagerGenerateSecretStringBuilder<GenerateStringKeyState> {
        SecretsManagerGenerateSecretStringBuilder {
            phantom_data: Default::default(),
            generate_string_key: Some(generate_string_key),
            exclude_characters: self.exclude_characters,
            exclude_lowercase: self.exclude_lowercase,
            exclude_numbers: self.exclude_numbers,
            exclude_punctuation: self.exclude_punctuation,
            exclude_uppercase: self.exclude_uppercase,
            include_space: self.include_space,
            password_length: self.password_length,
            require_each_included_type: self.require_each_included_type,
            secret_string_template: None,
        }
    }

    #[must_use]
    pub fn build(self) -> GenerateSecretString {
        self.build_internal()
    }
}

impl SecretsManagerGenerateSecretStringBuilder<GenerateStringKeyState> {
    pub fn secret_string_template(self, secret_string_template: Value) -> SecretsManagerGenerateSecretStringBuilder<SecretStringTemplateState> {
        SecretsManagerGenerateSecretStringBuilder {
            phantom_data: Default::default(),
            secret_string_template: Some(secret_string_template.to_string()),
            generate_string_key: self.generate_string_key,
            exclude_characters: self.exclude_characters,
            exclude_lowercase: self.exclude_lowercase,
            exclude_numbers: self.exclude_numbers,
            exclude_punctuation: self.exclude_punctuation,
            exclude_uppercase: self.exclude_uppercase,
            include_space: self.include_space,
            password_length: self.password_length,
            require_each_included_type: self.require_each_included_type,
        }
    }
}

impl SecretsManagerGenerateSecretStringBuilder<SecretStringTemplateState> {
    #[must_use]
    pub fn build(self) -> GenerateSecretString {
        self.build_internal()
    }
}