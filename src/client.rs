use log::info;
use tonic::transport::Channel;

use crate::error::Result;
use crate::parser::parse_state_from_json;
use crate::vehicle_shadow::signal_service_client::SignalServiceClient;
use crate::vehicle_shadow::{
    GetRequest, GetResponse, SetRequest, SetResponse, SetSignalRequest, SubscribeRequest,
    SubscribeResponse, UnsubscribeRequest, UnsubscribeResponse,
};

/// High-level client for the Vehicle Signal Shadow service
pub struct VehicleShadowClient {
    client: SignalServiceClient<Channel>,
}

impl VehicleShadowClient {
    /// Create a new client connected to the specified server
    pub async fn connect(server_url: &str) -> Result<Self> {
        info!("Connecting to Vehicle Signal Shadow server: {}", server_url);

        let channel = Channel::from_shared(server_url.to_string())?
            .connect()
            .await?;

        let client = SignalServiceClient::new(channel);

        Ok(VehicleShadowClient { client })
    }

    /// Get signals by their paths
    pub async fn get_signals(&mut self, paths: Vec<String>) -> Result<GetResponse> {
        info!("Getting signals: {:?}", paths);

        let request = tonic::Request::new(GetRequest { paths });
        let response = self.client.get(request).await?;

        Ok(response.into_inner())
    }

    /// Set a single signal value
    pub async fn set_signal(&mut self, path: String, value_json: &str) -> Result<SetResponse> {
        info!("Setting signal {} to value: {}", path, value_json);

        let state = parse_state_from_json(value_json)?;

        let set_request = SetSignalRequest {
            path: path.clone(),
            state: Some(state),
        };

        let request = tonic::Request::new(SetRequest {
            signals: vec![set_request],
        });

        let response = self.client.set(request).await?;

        Ok(response.into_inner())
    }

    /// Set multiple signal values
    pub async fn set_signals(&mut self, signals: Vec<(String, String)>) -> Result<SetResponse> {
        info!("Setting {} signals", signals.len());

        let mut set_requests = Vec::new();

        for (path, value_json) in signals {
            let state = parse_state_from_json(&value_json)?;
            set_requests.push(SetSignalRequest {
                path,
                state: Some(state),
            });
        }

        let request = tonic::Request::new(SetRequest {
            signals: set_requests,
        });

        let response = self.client.set(request).await?;

        Ok(response.into_inner())
    }

    /// Subscribe to signal changes
    pub async fn subscribe(
        &mut self,
        paths: Vec<String>,
    ) -> Result<tonic::codec::Streaming<SubscribeResponse>> {
        info!("Subscribing to signals: {:?}", paths);

        let request = tonic::Request::new(SubscribeRequest { paths });
        let response = self.client.subscribe(request).await?;

        Ok(response.into_inner())
    }

    /// Unsubscribe from signal changes
    pub async fn unsubscribe(&mut self, paths: Vec<String>) -> Result<UnsubscribeResponse> {
        info!("Unsubscribing from signals: {:?}", paths);

        let request = tonic::Request::new(UnsubscribeRequest { paths });
        let response = self.client.unsubscribe(request).await?;

        Ok(response.into_inner())
    }

    /// Get the underlying gRPC client for advanced usage
    pub fn get_client(&mut self) -> &mut SignalServiceClient<Channel> {
        &mut self.client
    }
}
