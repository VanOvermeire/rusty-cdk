use syn::Error;
use syn::__private::Span;

pub(crate) struct StringRequirements {
    pub(crate) not_empty: bool,
    pub(crate) allowed_chars: Vec<char>,
}

impl StringRequirements {
    pub(crate) fn not_empty(allowed_chars: Vec<char>) -> StringRequirements {
        Self {
            not_empty: true,
            allowed_chars,
        }
    }
}

pub(crate) fn check_string_requirements(value: &str, span: Span, requirements: StringRequirements) -> Option<Error> {
    if requirements.not_empty && value.is_empty() {
        return Some(Error::new(span, "value should not be blank".to_string()));
    }

    if value.chars().any(|c| !c.is_alphanumeric() && !requirements.allowed_chars.contains(&c)) {
        return Some(Error::new(
            span,
            format!("value should only contain alphanumeric characters and {:?}", requirements.allowed_chars),
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use syn::__private::Span;
    use crate::strings::{check_string_requirements, StringRequirements};

    #[test]
    fn should_return_empty_when_string_contains_only_alphanumeric_chars() {
        let requirements = StringRequirements {
            not_empty: true,
            allowed_chars: vec![],
        };

        let output = check_string_requirements("valid", Span::call_site(), requirements);

        assert!(output.is_none());
    }

    #[test]
    fn should_return_empty_when_string_is_empty_and_non_empty_not_required() {
        let requirements = StringRequirements {
            not_empty: false,
            allowed_chars: vec![],
        };

        let output = check_string_requirements("", Span::call_site(), requirements);

        assert!(output.is_none());
    }

    #[test]
    fn should_return_empty_when_string_contains_allowed_special_chars() {
        let requirements = StringRequirements {
            not_empty: true,
            allowed_chars: vec!['-', '_'],
        };

        let output = check_string_requirements("valid-name_123", Span::call_site(), requirements);

        assert!(output.is_none());
    }
    
    #[test]
    fn should_return_error_when_string_contains_invalid_char() {
        let requirements = StringRequirements {
            not_empty: true,
            allowed_chars: vec!['_'],
        };
        
        let output = check_string_requirements("invalid-hyphen", Span::call_site(), requirements);

        assert!(output.is_some());
    }

    #[test]
    fn should_return_error_when_string_is_empty_and_non_empty_required() {
        let requirements = StringRequirements {
            not_empty: true,
            allowed_chars: vec![],
        };

        let output = check_string_requirements("", Span::call_site(), requirements);

        assert_eq!(output.unwrap().to_string(), "value should not be blank");
    }

    #[test]
    fn should_return_error_when_string_contains_disallowed_special_char() {
        let requirements = StringRequirements {
            not_empty: true,
            allowed_chars: vec!['-'],
        };

        let output = check_string_requirements("invalid_underscore", Span::call_site(), requirements);

        assert!(output.is_some());
    }

    #[test]
    fn should_reject_spaces() {
        let requirements = StringRequirements {
            not_empty: true,
            allowed_chars: vec![],
        };

        let output = check_string_requirements("invalid name", Span::call_site(), requirements);

        assert!(output.is_some());
    }

    #[test]
    fn should_reject_special_chars_when_not_allowed() {
        let requirements = StringRequirements {
            not_empty: true,
            allowed_chars: vec![],
        };

        let output = check_string_requirements("invalid@email.com", Span::call_site(), requirements);

        assert!(output.is_some());
    }
}