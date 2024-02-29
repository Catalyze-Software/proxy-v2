use canister_types::models::{
    api_error::ApiError,
    date_range::DateRange,
    validation::{ValidateField, ValidationResponse, ValidationType},
};
use email_address::EmailAddress;
use ic_cdk::api::time;
use std::str::FromStr;

pub struct Validator {
    fields: Vec<ValidateField>,
}

impl Validator {
    pub fn new(fields: Vec<ValidateField>) -> Self {
        Validator { fields }
    }

    pub fn validate(&self) -> Result<(), ApiError> {
        let mut errors: Vec<ValidationResponse> = vec![];

        self.fields.iter().for_each(|f| {
            if let Err(err) = Self::validate_field(f) {
                errors.push(err);
            }
        });

        if errors.len() > 0 {
            return Err(ApiError::validation_response(errors));
        }

        return Ok(());
    }

    fn validate_field(validation_field: &ValidateField) -> Result<(), ValidationResponse> {
        let ValidateField(validation_type, field) = validation_field;

        use ValidationType::*;
        match validation_type {
            None => Ok(()),
            StringLength(value, min, max) => Self::validate_string_length(value, min, max, field),
            DateRange(value) => Self::validate_date_range(value, field),
            Email(value) => Self::validate_email(value, field),
            Count(value, min, max) => Self::validate_count(value, min, max, field),
        }
    }

    fn validate_string_length(
        value: &String,
        min: &usize,
        max: &usize,
        field: &String,
    ) -> Result<(), ValidationResponse> {
        if value.len() < *min {
            return Err(ValidationResponse {
                field: field.to_string(),
                message: format!("Minimum required length is {}", min),
            });
        } else if value.len() > *max {
            return Err(ValidationResponse {
                field: field.to_string(),
                message: format!("Maximum length is {}", max),
            });
        } else {
            return Ok(());
        }
    }

    fn validate_count(
        value: &usize,
        min: &usize,
        max: &usize,
        field: &String,
    ) -> Result<(), ValidationResponse> {
        if value < min {
            return Err(ValidationResponse {
                field: field.to_string(),
                message: format!("Minimum size length is {}", min),
            });
        } else if value > max {
            return Err(ValidationResponse {
                field: field.to_string(),
                message: format!("Maximum size is {}", max),
            });
        } else {
            return Ok(());
        }
    }

    fn validate_email(value: &String, field: &String) -> Result<(), ValidationResponse> {
        let email = EmailAddress::from_str(&value.as_str());

        match email {
            Ok(_email) => Ok(()),
            Err(err) => Err(ValidationResponse {
                field: field.to_string(),
                message: err.to_string(),
            }),
        }
    }

    fn validate_date_range(value: &DateRange, field: &String) -> Result<(), ValidationResponse> {
        if value.start_date() > value.end_date() {
            return Err(ValidationResponse {
                field: field.to_string(),
                message: format!("The start_date is after the end_date"),
            });
        } else if value.start_date() < time() {
            return Err(ValidationResponse {
                field: field.to_string(),
                message: format!("The start_date can't be in the past"),
            });
        } else {
            return Ok(());
        }
    }
}
