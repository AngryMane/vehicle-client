use anyhow::anyhow;
use serde_json;

use crate::error::{ClientError, Result};
use crate::vehicle_shadow::{BoolArray, Int32Array, State, StringArray, Value};

/// Parse a JSON string into a State object
pub fn parse_state_from_json(json_str: &str) -> Result<State> {
    let json_value: serde_json::Value = serde_json::from_str(json_str)?;

    match json_value {
        serde_json::Value::Object(obj) => {
            let value = if let Some(value_json) = obj.get("value") {
                Some(parse_value_from_json(&value_json.to_string())?)
            } else {
                None
            };

            let capability = obj.get("capability").and_then(|v| v.as_bool());

            let availability = obj.get("availability").and_then(|v| v.as_bool());

            let reserved = obj
                .get("reserved")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            Ok(State {
                value,
                capability,
                availability,
                reserved,
            })
        }
        _ => {
            // 単純な値の場合は、valueとして扱う
            let value = parse_value_from_json(json_str)?;
            Ok(State {
                value: Some(value),
                capability: None,
                availability: None,
                reserved: None,
            })
        }
    }
}

/// Parse a JSON string into a Value object
pub fn parse_value_from_json(json_str: &str) -> Result<Value> {
    let json_value: serde_json::Value = serde_json::from_str(json_str)?;

    match json_value {
        serde_json::Value::Bool(b) => Ok(Value {
            value: Some(crate::vehicle_shadow::value::Value::BoolValue(b)),
        }),
        serde_json::Value::String(s) => Ok(Value {
            value: Some(crate::vehicle_shadow::value::Value::StringValue(s)),
        }),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                    Ok(Value {
                        value: Some(crate::vehicle_shadow::value::Value::Int32Value(i as i32)),
                    })
                } else {
                    Ok(Value {
                        value: Some(crate::vehicle_shadow::value::Value::Int64Value(i)),
                    })
                }
            } else if let Some(f) = n.as_f64() {
                Ok(Value {
                    value: Some(crate::vehicle_shadow::value::Value::DoubleValue(f)),
                })
            } else {
                Err(ClientError::InvalidInput(
                    "Invalid number format".to_string(),
                ))
            }
        }
        serde_json::Value::Array(arr) => {
            // 配列の最初の要素の型を基に配列型を決定
            if arr.is_empty() {
                return Err(ClientError::InvalidInput(
                    "Empty arrays are not supported".to_string(),
                ));
            }

            match &arr[0] {
                serde_json::Value::Bool(_) => {
                    let bool_values: std::result::Result<Vec<bool>, _> = arr
                        .iter()
                        .map(|v| v.as_bool().ok_or_else(|| anyhow!("Invalid boolean value")))
                        .collect();
                    let bool_values =
                        bool_values.map_err(|e| ClientError::InvalidInput(e.to_string()))?;
                    Ok(Value {
                        value: Some(crate::vehicle_shadow::value::Value::BoolArrayValue(
                            BoolArray {
                                values: bool_values,
                            },
                        )),
                    })
                }
                serde_json::Value::String(_) => {
                    let string_values: std::result::Result<Vec<String>, _> = arr
                        .iter()
                        .map(|v| {
                            v.as_str()
                                .map(|s| s.to_string())
                                .ok_or_else(|| anyhow!("Invalid string value"))
                        })
                        .collect();
                    let string_values =
                        string_values.map_err(|e| ClientError::InvalidInput(e.to_string()))?;
                    Ok(Value {
                        value: Some(crate::vehicle_shadow::value::Value::StringArrayValue(
                            StringArray {
                                values: string_values,
                            },
                        )),
                    })
                }
                serde_json::Value::Number(_) => {
                    // 数値配列の場合はInt32Arrayとして扱う
                    let int_values: std::result::Result<Vec<i32>, _> = arr
                        .iter()
                        .map(|v| {
                            v.as_i64()
                                .and_then(|i| i32::try_from(i).ok())
                                .ok_or_else(|| anyhow!("Invalid integer value"))
                        })
                        .collect();
                    let int_values =
                        int_values.map_err(|e| ClientError::InvalidInput(e.to_string()))?;
                    Ok(Value {
                        value: Some(crate::vehicle_shadow::value::Value::Int32ArrayValue(
                            Int32Array { values: int_values },
                        )),
                    })
                }
                _ => Err(ClientError::InvalidInput(
                    "Unsupported array element type".to_string(),
                )),
            }
        }
        _ => Err(ClientError::InvalidInput(
            "Unsupported JSON value type".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_value() {
        let result = parse_value_from_json("42").unwrap();
        assert!(matches!(
            result.value,
            Some(crate::vehicle_shadow::value::Value::Int32Value(42))
        ));
    }

    #[test]
    fn test_parse_string_value() {
        let result = parse_value_from_json("\"hello\"").unwrap();
        assert!(
            matches!(result.value, Some(crate::vehicle_shadow::value::Value::StringValue(ref s)) if s == "hello")
        );
    }

    #[test]
    fn test_parse_boolean_value() {
        let result = parse_value_from_json("true").unwrap();
        assert!(matches!(
            result.value,
            Some(crate::vehicle_shadow::value::Value::BoolValue(true))
        ));
    }

    #[test]
    fn test_parse_array_value() {
        let result = parse_value_from_json("[1, 2, 3]").unwrap();
        assert!(matches!(
            result.value,
            Some(crate::vehicle_shadow::value::Value::Int32ArrayValue(_))
        ));
    }

    #[test]
    fn test_parse_state_with_value() {
        let result = parse_state_from_json("{\"value\": 42}").unwrap();
        assert!(result.value.is_some());
        assert!(result.capability.is_none());
        assert!(result.availability.is_none());
    }

    #[test]
    fn test_parse_state_with_all_fields() {
        let result = parse_state_from_json("{\"value\": 42, \"capability\": true, \"availability\": false, \"reserved\": \"test\"}").unwrap();
        assert!(result.value.is_some());
        assert_eq!(result.capability, Some(true));
        assert_eq!(result.availability, Some(false));
        assert_eq!(result.reserved, Some("test".to_string()));
    }
}
