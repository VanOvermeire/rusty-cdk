pub(crate) struct StringRequirements {
    check_chars: bool,
    min_length: usize,
    max_length: Option<usize>,
    prefix: Option<String>,
    allowed_chars: Vec<char>,
}

impl StringRequirements {
    pub(crate) fn not_empty_with_allowed_chars(specific_allowed_chars: Vec<char>) -> Self {
        Self {
            check_chars: true,
            min_length: 1,
            max_length: None,
            prefix: None,
            allowed_chars: specific_allowed_chars,
        }
    }

    pub(crate) fn not_empty_prefix(prefix: &str) -> Self {
        Self {
            check_chars: false,
            min_length: 1,
            max_length: None,
            prefix: Some(prefix.to_string()),
            allowed_chars: vec![],
        }
    }

    pub(crate) fn with_max_length(self, max_length: usize) -> Self {
        Self {
            max_length: Some(max_length),
            ..self
        }
    }
}

pub(crate) fn validate_string(value: &str, requirements: StringRequirements) -> Result<(), String> {
    if value.len() < requirements.min_length {
        return Err(format!("min allowed length is {} (was {})", requirements.min_length, value.len()));
    }

    if let Some(max) = requirements.max_length
        && value.len() > max
    {
        return Err(format!("max allowed length is {} (was {})", max, value.len()));
    }

    if let Some(prefix) = requirements.prefix
        && !value.starts_with(&prefix)
    {
        return Err(format!("value `{}` does not contain required prefix `{}`", value, prefix));
    }

    if requirements.check_chars
        && value
            .chars()
            .any(|c| !c.is_alphanumeric() && !requirements.allowed_chars.contains(&c))
    {
        return Err(format!(
            "value should only contain alphanumeric characters and {:?}",
            requirements.allowed_chars
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::strings::{StringRequirements, validate_string};

    #[test]
    fn should_return_empty_when_string_contains_prefix() {
        let requirements = StringRequirements::not_empty_prefix("some-prefix");

        let output = validate_string("some-prefix-and-more-text", requirements);

        assert!(output.is_ok());
    }

    #[test]
    fn should_return_error_when_string_does_not_contain_prefix() {
        let requirements = StringRequirements::not_empty_prefix("some-prefix");

        let output = validate_string("just-text", requirements);

        assert!(output.is_err());
    }

    #[test]
    fn should_return_empty_when_string_contains_only_alphanumeric_chars_and_there_are_no_additional_allowed_chars() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec![]);

        let output = validate_string("valid", requirements);

        assert!(output.is_ok());
    }

    #[test]
    fn should_return_empty_when_string_contains_allowed_special_chars() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec!['-', '_']);

        let output = validate_string("valid-name_123", requirements);

        assert!(output.is_ok());
    }

    #[test]
    fn should_return_error_when_string_contains_invalid_char() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec!['_']);

        let output = validate_string("invalid-hyphen", requirements);

        assert!(output.is_err());
    }

    #[test]
    fn should_return_error_when_string_is_empty() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec![]);

        let output = validate_string("", requirements);

        assert_eq!(output.unwrap_err().to_string(), "min allowed length is 1 (was 0)");
    }

    #[test]
    fn should_return_error_when_string_is_too_long() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec![]).with_max_length(2);

        let output = validate_string("too long", requirements);

        assert_eq!(output.unwrap_err().to_string(), "max allowed length is 2 (was 8)");
    }

    #[test]
    fn should_return_error_when_string_contains_disallowed_special_char() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec!['-']);

        let output = validate_string("invalid_underscore", requirements);

        assert!(output.is_err());
    }

    #[test]
    fn should_reject_spaces() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec![]);

        let output = validate_string("invalid name", requirements);

        assert!(output.is_err());
    }

    #[test]
    fn should_reject_special_chars_when_not_allowed() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec![]);

        let output = validate_string("invalid@email.com", requirements);

        assert!(output.is_err());
    }
}
