use ValidatorCondition;
use regex::Regex;

lazy_static! {
    // This is the regular expression from RFC 5322
    static ref EMAIL_REGEX: Regex = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap();
}

/// Validator that checks whether the given input is a valid email address according to RFC 5322
///
/// # Examples
///
/// ```
/// use gild::ValidationChain;
/// use gild::validators;
///
/// ValidationChain::new()
///    .add(validators::IsEmail::new())
///    .validate(String::from("john.doe@example.com"))
///    .is_ok();
/// ```
pub struct IsEmail;

impl ValidatorCondition for IsEmail {
    fn validate(&self, input: String) -> bool {
        // if this is the case we're 100% sure it's not an email, so we don't have to waste cpu power
        if input.is_empty() || !input.contains("@") {
            return false
        }

        if EMAIL_REGEX.is_match(input.as_str()) {
            return true
        } else {
            return false
        }
    }

    fn get_err_message(&self) -> String {
        format!("Input is not a valid e-mail address")
    }
}

impl IsEmail {
    pub fn new() -> Box<Self> {
        Box::new(IsEmail)
    }
}
