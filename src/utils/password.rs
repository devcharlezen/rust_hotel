use validator::ValidationError;

use crate::{ utils::response::AppError};

pub fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric());

    if has_upper && has_lower && has_digit && has_special {
        Ok(())
    } else {
        Err(ValidationError::new("weak_password").with_message(
            "Password must contain an uppercase letter, a lowercase letter, a number and a special character".into(),
        ))
    }
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(hashed)
}

pub fn compare_password(plain: &str, hashed: &str) -> Result<bool, AppError> {
    let is_valid = bcrypt::verify(plain, hashed).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(is_valid)
}

