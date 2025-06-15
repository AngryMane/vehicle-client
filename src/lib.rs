//! Vehicle Signal Shadow Client Library
//!
//! This library provides a high-level client interface for the Vehicle Signal Shadow gRPC service.
//! It includes utilities for connecting to the service, parsing JSON data, and formatting responses.

pub mod client;
pub mod error;
pub mod formatter;
pub mod parser;

// Re-export the generated proto types
pub mod vehicle_shadow {
    tonic::include_proto!("vehicle_shadow");
}

pub use client::VehicleShadowClient;
pub use error::{ClientError, Result};
pub use formatter::{format_value, format_signal};
pub use parser::{parse_state_from_json, parse_value_from_json};

// Re-export commonly used types
pub use vehicle_shadow::{
    Config, GetRequest, GetResponse, LeafType, SetRequest, SetResponse, SetResult,
    SetSignalRequest, Signal, State, SubscribeRequest, SubscribeResponse, UnsubscribeRequest,
    UnsubscribeResponse, Value, ValueType,
};
