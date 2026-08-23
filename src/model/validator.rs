use crate::core::error::VellaError;
use crate::model::field::{Field, FieldType};
use serde_json::Value;

pub struct FieldValidator;

impl FieldValidator {
    /// Validate an incoming JSON payload against a field definition
    pub fn validate_field(field: &Field, val: Option<&Value>) -> Result<(), VellaError> {
        // Required check
        if field.required && (val.is_none() || val == Some(&Value::Null) || val == Some(&Value::String(String::new()))) {
            return Err(VellaError::Validation(format!(
                "Field '{}' ({}) is required",
                field.name, field.display_name
            )));
        }

        if let Some(v) = val {
            if v.is_null() {
                return Ok(());
            }

            match &field.field_type {
                FieldType::Integer | FieldType::ForeignKey { .. } => {
                    if !v.is_number() && !v.is_i64() {
                        if let Some(s) = v.as_str() {
                            if s.parse::<i64>().is_err() {
                                return Err(VellaError::Validation(format!("Field '{}' must be an integer", field.name)));
                            }
                        } else {
                            return Err(VellaError::Validation(format!("Field '{}' must be an integer", field.name)));
                        }
                    }
                }
                FieldType::Float | FieldType::Money { .. } | FieldType::ProgressBar { .. } => {
                    if !v.is_number() {
                        if let Some(s) = v.as_str() {
                            if s.parse::<f64>().is_err() {
                                return Err(VellaError::Validation(format!("Field '{}' must be a numeric value", field.name)));
                            }
                        } else {
                            return Err(VellaError::Validation(format!("Field '{}' must be a numeric value", field.name)));
                        }
                    }
                }
                FieldType::Boolean => {
                    if !v.is_boolean() && v.as_i64().is_none() {
                        return Err(VellaError::Validation(format!("Field '{}' must be a boolean", field.name)));
                    }
                }
                FieldType::Enum { choices } => {
                    if let Some(s) = v.as_str() {
                        if !choices.contains(&s.to_string()) {
                            return Err(VellaError::Validation(format!(
                                "Invalid choice '{}' for field '{}'. Allowed: {:?}",
                                s, field.name, choices
                            )));
                        }
                    }
                }
                FieldType::Vector { dimensions } => {
                    match v {
                        Value::Array(arr) => {
                            if arr.len() != *dimensions {
                                return Err(VellaError::Validation(format!(
                                    "Field '{}' requires exactly {} vector dimensions, but got {}",
                                    field.name, dimensions, arr.len()
                                )));
                            }
                            for (idx, item) in arr.iter().enumerate() {
                                if !item.is_number() {
                                    return Err(VellaError::Validation(format!(
                                        "Field '{}' vector element at index {} is not a valid number",
                                        field.name, idx
                                    )));
                                }
                            }
                        }
                        Value::String(s) => {
                            // If encoded as comma-separated or json string
                            if let Ok(Value::Array(arr)) = serde_json::from_str::<Value>(s) {
                                if arr.len() != *dimensions {
                                    return Err(VellaError::Validation(format!(
                                        "Field '{}' requires exactly {} vector dimensions, but got {}",
                                        field.name, dimensions, arr.len()
                                    )));
                                }
                            }
                        }
                        _ => {
                            return Err(VellaError::Validation(format!(
                                "Field '{}' must be a vector array of {} floating point numbers",
                                field.name, dimensions
                            )));
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
