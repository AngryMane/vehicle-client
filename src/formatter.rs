use crate::vehicle_shadow::Value;

/// Format a Value object into a human-readable string
pub fn format_value(value: &Value) -> String {
    match &value.value {
        Some(crate::vehicle_shadow::value::Value::BoolValue(v)) => format!("{}", v),
        Some(crate::vehicle_shadow::value::Value::StringValue(v)) => format!("\"{}\"", v),
        Some(crate::vehicle_shadow::value::Value::Int8Value(v)) => format!("{}", v),
        Some(crate::vehicle_shadow::value::Value::Int16Value(v)) => format!("{}", v),
        Some(crate::vehicle_shadow::value::Value::Int32Value(v)) => format!("{}", v),
        Some(crate::vehicle_shadow::value::Value::Int64Value(v)) => format!("{}", v),
        Some(crate::vehicle_shadow::value::Value::Uint8Value(v)) => format!("{}", v),
        Some(crate::vehicle_shadow::value::Value::Uint16Value(v)) => format!("{}", v),
        Some(crate::vehicle_shadow::value::Value::Uint32Value(v)) => format!("{}", v),
        Some(crate::vehicle_shadow::value::Value::Uint64Value(v)) => format!("{}", v),
        Some(crate::vehicle_shadow::value::Value::FloatValue(v)) => format!("{}", v),
        Some(crate::vehicle_shadow::value::Value::DoubleValue(v)) => format!("{}", v),
        Some(crate::vehicle_shadow::value::Value::BoolArrayValue(v)) => format!("{:?}", v.values),
        Some(crate::vehicle_shadow::value::Value::StringArrayValue(v)) => format!("{:?}", v.values),
        Some(crate::vehicle_shadow::value::Value::Int8ArrayValue(v)) => format!("{:?}", v.values),
        Some(crate::vehicle_shadow::value::Value::Int16ArrayValue(v)) => format!("{:?}", v.values),
        Some(crate::vehicle_shadow::value::Value::Int32ArrayValue(v)) => format!("{:?}", v.values),
        Some(crate::vehicle_shadow::value::Value::Int64ArrayValue(v)) => format!("{:?}", v.values),
        Some(crate::vehicle_shadow::value::Value::Uint8ArrayValue(v)) => format!("{:?}", v.values),
        Some(crate::vehicle_shadow::value::Value::Uint16ArrayValue(v)) => format!("{:?}", v.values),
        Some(crate::vehicle_shadow::value::Value::Uint32ArrayValue(v)) => format!("{:?}", v.values),
        Some(crate::vehicle_shadow::value::Value::Uint64ArrayValue(v)) => format!("{:?}", v.values),
        Some(crate::vehicle_shadow::value::Value::FloatArrayValue(v)) => format!("{:?}", v.values),
        Some(crate::vehicle_shadow::value::Value::DoubleArrayValue(v)) => format!("{:?}", v.values),
        None => "NAN".to_string(),
    }
}

/// Format a signal for display
pub fn format_signal(signal: &crate::vehicle_shadow::Signal) -> String {
    let mut output = format!("Path: {}\n", signal.path);

    if let Some(state) = &signal.state {
        if let Some(value) = &state.value {
            output.push_str(&format!("  Value: {}\n", format_value(value)));
        }
        output.push_str(&format!(
            "  Capability: {}\n",
            state.capability.unwrap_or(false)
        ));
        output.push_str(&format!(
            "  Availability: {}\n",
            state.availability.unwrap_or(false)
        ));
    }

    if let Some(config) = &signal.config {
        output.push_str(&format!("  Type: {:?}\n", config.leaf_type));
        output.push_str(&format!("  Data Type: {:?}\n", config.data_type));
        if let Some(unit) = &config.unit {
            output.push_str(&format!("  Unit: {}\n", unit));
        }
        if let Some(description) = &config.description {
            output.push_str(&format!("  Description: {}\n", description));
        }
        output.push_str(&format!("  End Point: {}\n", config.end_point));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bool_value() {
        let value = Value {
            value: Some(crate::vehicle_shadow::value::Value::BoolValue(true)),
        };
        assert_eq!(format_value(&value), "true");
    }

    #[test]
    fn test_format_string_value() {
        let value = Value {
            value: Some(crate::vehicle_shadow::value::Value::StringValue(
                "hello".to_string(),
            )),
        };
        assert_eq!(format_value(&value), "\"hello\"");
    }

    #[test]
    fn test_format_int_value() {
        let value = Value {
            value: Some(crate::vehicle_shadow::value::Value::Int32Value(42)),
        };
        assert_eq!(format_value(&value), "42");
    }

    #[test]
    fn test_format_array_value() {
        let value = Value {
            value: Some(crate::vehicle_shadow::value::Value::Int32ArrayValue(
                crate::vehicle_shadow::Int32Array {
                    values: vec![1, 2, 3],
                },
            )),
        };
        assert_eq!(format_value(&value), "[1, 2, 3]");
    }

    #[test]
    fn test_format_nan_value() {
        let value = Value { value: None };
        assert_eq!(format_value(&value), "NAN");
    }

    #[test]
    fn test_format_signal_with_end_point() {
        use crate::vehicle_shadow::{Config, LeafType, Signal, State, Value, ValueType};
        
        let signal = Signal {
            path: "Vehicle.Speed".to_string(),
            state: Some(State {
                value: Some(Value {
                    value: Some(crate::vehicle_shadow::value::Value::DoubleValue(60.5)),
                }),
                capability: Some(true),
                availability: Some(true),
                reserved: None,
            }),
            config: Some(Config {
                leaf_type: LeafType::Sensor as i32,
                data_type: ValueType::TypeDouble as i32,
                deprecation: None,
                unit: Some("km/h".to_string()),
                min: None,
                max: None,
                description: Some("Vehicle speed".to_string()),
                comment: None,
                allowd: vec![],
                default: None,
                end_point: "vehicle.speed".to_string(),
            }),
        };

        let formatted = format_signal(&signal);
        assert!(formatted.contains("End Point: vehicle.speed"));
        assert!(formatted.contains("Path: Vehicle.Speed"));
        assert!(formatted.contains("Value: 60.5"));
        assert!(formatted.contains("Unit: km/h"));
        assert!(formatted.contains("Description: Vehicle speed"));
    }
}
