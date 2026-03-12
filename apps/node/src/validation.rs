use crate::error::AppError;
use url::Url;

/// Validate a URL string — must be http or https
pub fn validate_url(field: &str, val: &str) -> Result<(), AppError> {
    match Url::parse(val) {
        Ok(u) if u.scheme() == "http" || u.scheme() == "https" => Ok(()),
        _ => Err(AppError::Validation(format!("'{field}' must be a valid http(s) URL"))),
    }
}

/// Validate a string is not empty
pub fn require_str(field: &str, val: &str) -> Result<(), AppError> {
    if val.trim().is_empty() {
        Err(AppError::Validation(format!("'{field}' is required")))
    } else {
        Ok(())
    }
}

/// Validate max string length
pub fn max_len(field: &str, val: &str, max: usize) -> Result<(), AppError> {
    if val.len() > max {
        Err(AppError::Validation(format!(
            "'{field}' must be at most {max} characters (got {})", val.len()
        )))
    } else {
        Ok(())
    }
}

/// Validate price_amount is positive
pub fn validate_price_amount(amount: Option<i64>) -> Result<(), AppError> {
    if let Some(a) = amount {
        if a < 0 {
            return Err(AppError::Validation("'price_amount' must be >= 0".into()));
        }
    }
    Ok(())
}

/// Validate price_model + amount consistency
pub fn validate_price(model: &str, amount: Option<i64>, currency: Option<&str>) -> Result<(), AppError> {
    if (model == "fixed" || model == "subscription") && amount.is_none() {
        return Err(AppError::Validation(
            format!("'price_amount' is required for price model '{model}'")
        ));
    }
    if amount.is_some() && currency.is_none() {
        return Err(AppError::Validation(
            "'price_currency' is required when 'price_amount' is set".into()
        ));
    }
    validate_price_amount(amount)?;
    Ok(())
}
