use std::borrow::Cow;
use validator::ValidationError;

pub const ALLOWED_SPECIAL_CHARS: &'static str = ",.-!?;:_@^*$%";

fn create_error(code: &'static str, message: impl Into<Cow<'static, str>>) -> ValidationError {
    let mut error = ValidationError::new(code);
    error.message = Some(message.into());
    error
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let (has_up, has_low, has_digit, has_spec) =
        password
            .chars()
            .fold((false, false, false, false), |(up, low, digit, spec), c| {
                (
                    up || c.is_ascii_uppercase(),
                    low || c.is_ascii_lowercase(),
                    digit || c.is_ascii_digit(),
                    spec || ALLOWED_SPECIAL_CHARS.contains(c),
                )
            });

    match (has_up, has_low, has_digit, has_spec) {
        (false, _, _, _) => Err(create_error(
            "missing_uppercase",
            "Password must contain at least 1 uppercase letter",
        )),
        (_, false, _, _) => Err(create_error(
            "missing_lowercase",
            "Password must contain at least 1 lowercase letter",
        )),
        (_, _, false, _) => Err(create_error(
            "missing_digit",
            "Password must contain at least 1 number",
        )),
        (_, _, _, false) => Err(create_error(
            "missing_special",
            format!(
                "Password must contain at least 1 special character: {}",
                ALLOWED_SPECIAL_CHARS,
            ),
        )),
        _ => Ok(()),
    }
}
