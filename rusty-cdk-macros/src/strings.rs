use syn::Error;
use syn::__private::Span;

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

pub(crate) fn check_string_requirements(value: &str, span: Span, requirements: StringRequirements) -> Result<(), Error> {
    if value.len() < requirements.min_length {
        return Err(Error::new(span, format!("min required length is {} (was {})", requirements.min_length, value.len())));
    }
    
    if let Some(max) = requirements.max_length && value.len() > max {
        return Err(Error::new(span, format!("max required length is {} (was {})", max, value.len())));
    }
    
    if let Some(prefix) = requirements.prefix && !value.starts_with(&prefix) {
        return Err(Error::new(span, format!("value `{}` does not contain required prefix `{}`", value, prefix)));
    }
    
    if requirements.check_chars && value.chars().any(|c| !c.is_alphanumeric() && !requirements.allowed_chars.contains(&c)) {
        return Err(Error::new(
            span,
            format!("value should only contain alphanumeric characters and {:?}", requirements.allowed_chars),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use syn::__private::Span;
    use crate::strings::{check_string_requirements, StringRequirements};

    #[test]
    fn should_return_empty_when_string_contains_prefix() {
        let requirements = StringRequirements::not_empty_prefix("some-prefix");
    
        let output = check_string_requirements("some-prefix-and-more-text", Span::call_site(), requirements);
    
        assert!(output.is_ok());
    }

    #[test]
    fn should_return_error_when_string_does_not_contain_prefix() {
    let requirements = StringRequirements::not_empty_prefix("some-prefix");
    
        let output = check_string_requirements("just-text", Span::call_site(), requirements);
    
        assert!(output.is_err());
    }

    #[test]
    fn should_return_empty_when_string_contains_only_alphanumeric_chars_and_there_are_no_additional_allowed_chars() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec![]);

        let output = check_string_requirements("valid", Span::call_site(), requirements);

        assert!(output.is_ok());
    }

    #[test]
    fn should_return_empty_when_string_contains_allowed_special_chars() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec!['-', '_']);

        let output = check_string_requirements("valid-name_123", Span::call_site(), requirements);

        assert!(output.is_ok());
    }
    
    #[test]
    fn should_return_error_when_string_contains_invalid_char() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec!['_']);
        
        let output = check_string_requirements("invalid-hyphen", Span::call_site(), requirements);

        assert!(output.is_err());
    }

    #[test]
    fn should_return_error_when_string_is_empty() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec![]);

        let output = check_string_requirements("", Span::call_site(), requirements);

        assert_eq!(output.unwrap_err().to_string(), "min required length is 1 (was 0)");
    }

    #[test]
    fn should_return_error_when_string_is_too_long() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec![]).with_max_length(2);

        let output = check_string_requirements("too long", Span::call_site(), requirements);

        assert_eq!(output.unwrap_err().to_string(), "max required length is 2 (was 8)");
    }

    #[test]
    fn should_return_error_when_string_contains_disallowed_special_char() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec!['-']);

        let output = check_string_requirements("invalid_underscore", Span::call_site(), requirements);

        assert!(output.is_err());
    }

    #[test]
    fn should_reject_spaces() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec![]);

        let output = check_string_requirements("invalid name", Span::call_site(), requirements);

        assert!(output.is_err());
    }

    #[test]
    fn should_reject_special_chars_when_not_allowed() {
        let requirements = StringRequirements::not_empty_with_allowed_chars(vec![]);

        let output = check_string_requirements("invalid@email.com", Span::call_site(), requirements);

        assert!(output.is_err());
    }
}