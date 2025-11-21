use syn::Error;
use syn::__private::Span;

pub(crate) struct StringRequirements {
    not_empty: bool,
    check_chars: bool,
    prefix: Option<String>,
    allowed_chars: Vec<char>,
}

impl StringRequirements {
    pub(crate) fn not_empty_allowed_chars(specific_allowed_chars: Vec<char>) -> Self {
        Self {
            not_empty: true,
            check_chars: true,
            prefix: None,
            allowed_chars: specific_allowed_chars,
        }
    }

    pub(crate) fn not_empty_prefix(prefix: &str) -> Self {
        Self {
            not_empty: true,
            check_chars: false,
            prefix: Some(prefix.to_string()),
            allowed_chars: vec![],
        }
    }
}

pub(crate) fn check_string_requirements(value: &str, span: Span, requirements: StringRequirements) -> Option<Error> {
    if requirements.not_empty && value.is_empty() {
        return Some(Error::new(span, "value should not be blank".to_string()));
    }
    
    if let Some(prefix) = requirements.prefix {
        if !value.starts_with(&prefix) {
            return Some(Error::new(span, format!("Value {} does not contain required prefix {}", value, prefix)));   
        }
    }
    
    if requirements.check_chars {
        if value.chars().any(|c| !c.is_alphanumeric() && !requirements.allowed_chars.contains(&c)) {
            return Some(Error::new(
                span,
                format!("value should only contain alphanumeric characters and {:?}", requirements.allowed_chars),
            ));
        }   
    }

    None
}

#[cfg(test)]
mod tests {
    use syn::__private::Span;
    use crate::strings::{check_string_requirements, StringRequirements};

    #[test]
    fn should_return_empty_when_string_contains_prefix() {
        let requirements = StringRequirements::not_empty_prefix("some-prefix");
    
        let output = check_string_requirements("some-prefix-and-more-text", Span::call_site(), requirements);
    
        assert!(output.is_none());
    }

    #[test]
    fn should_return_error_when_string_does_not_contain_prefix() {
    let requirements = StringRequirements::not_empty_prefix("some-prefix");
    
        let output = check_string_requirements("just-text", Span::call_site(), requirements);
    
        assert!(output.is_some());
    }

    #[test]
    fn should_return_empty_when_string_contains_only_alphanumeric_chars_and_there_are_no_additional_allowed_chars() {
        let requirements = StringRequirements::not_empty_allowed_chars(vec![]);

        let output = check_string_requirements("valid", Span::call_site(), requirements);

        assert!(output.is_none());
    }

    #[test]
    fn should_return_empty_when_string_is_empty_and_non_empty_not_required() {
        let requirements = StringRequirements {
            not_empty: false,
            check_chars: true,
            prefix: None,
            allowed_chars: vec![],
        };

        let output = check_string_requirements("", Span::call_site(), requirements);

        assert!(output.is_none());
    }

    #[test]
    fn should_return_empty_when_string_contains_allowed_special_chars() {
        let requirements = StringRequirements::not_empty_allowed_chars(vec!['-', '_']);

        let output = check_string_requirements("valid-name_123", Span::call_site(), requirements);

        assert!(output.is_none());
    }
    
    #[test]
    fn should_return_error_when_string_contains_invalid_char() {
        let requirements = StringRequirements::not_empty_allowed_chars(vec!['_']);
        
        let output = check_string_requirements("invalid-hyphen", Span::call_site(), requirements);

        assert!(output.is_some());
    }

    #[test]
    fn should_return_error_when_string_is_empty_and_non_empty_required() {
        let requirements = StringRequirements::not_empty_allowed_chars(vec![]);

        let output = check_string_requirements("", Span::call_site(), requirements);

        assert_eq!(output.unwrap().to_string(), "value should not be blank");
    }

    #[test]
    fn should_return_error_when_string_contains_disallowed_special_char() {
        let requirements = StringRequirements::not_empty_allowed_chars(vec!['-']);

        let output = check_string_requirements("invalid_underscore", Span::call_site(), requirements);

        assert!(output.is_some());
    }

    #[test]
    fn should_reject_spaces() {
        let requirements = StringRequirements::not_empty_allowed_chars(vec![]);

        let output = check_string_requirements("invalid name", Span::call_site(), requirements);

        assert!(output.is_some());
    }

    #[test]
    fn should_reject_special_chars_when_not_allowed() {
        let requirements = StringRequirements::not_empty_allowed_chars(vec![]);

        let output = check_string_requirements("invalid@email.com", Span::call_site(), requirements);

        assert!(output.is_some());
    }
}