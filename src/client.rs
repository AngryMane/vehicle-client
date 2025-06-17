use log::info;
use tonic::transport::Channel;
use std::collections::HashMap;

use crate::error::{Result, ClientError};
use crate::vehicle_shadow::signal_service_client::SignalServiceClient;
use crate::vehicle_shadow::{
    GetRequest, GetResponse, SetRequest, SetResponse, SetSignalRequest, SubscribeRequest, State,
    SubscribeResponse, UnsubscribeRequest, UnsubscribeResponse,
};

/// High-level client for the Vehicle Signal Shadow service
pub struct VehicleShadowClient {
    clients: Vec<(String, SignalServiceClient<Channel>)>,
    //client: SignalServiceClient<Channel>,
}

impl VehicleShadowClient {

    pub async fn create() -> Result<Self> {
        Ok( VehicleShadowClient { clients: [].to_vec() } )
    }
    pub async fn connect(&mut self, server_url: &str, path: String) -> Result<()> {
        info!("Connecting to Vehicle Signal Shadow server: {}", server_url);

        let channel = Channel::from_shared(server_url.to_string())?
            .connect()
            .await?;

        let client = SignalServiceClient::new(channel);
        self.clients.push((path, client));

        Ok(())
    }

    /// Get signals by their paths
    pub async fn get_signals(&mut self, paths: Vec<String>) -> Result<GetResponse> {
        info!("Getting signals: {:?}", paths);

        // TODO: enable invoking multiple set
        let mut ret = GetResponse { signals: [].to_vec(), success: true, error_message: String::from("") };
        for path in paths {
            let client = self.get_target_client(path.clone());
            if client.is_none(){
                continue;
            }
            let client = client.unwrap();
            let request = tonic::Request::new(GetRequest { paths: [ path.clone() ].to_vec() });

            let response = client.get(request).await?.into_inner();
            ret.signals.append(&mut response.signals.clone());
        }

        Ok(ret)
    }

    /// Set multiple signal values
    pub async fn set_signals(&mut self, signals: Vec<(String, State)>) -> Result<SetResponse> {
        info!("Setting {} signals", signals.len());

        // TODO: enable invoking multiple set
        let mut ret = SetResponse { results: todo!(), success: true, error_message: String::from("")  };
        for (path, state)in signals {
            let client = self.get_target_client(path.clone());
            if client.is_none(){
                continue;
            }

            let mut set_requests = Vec::new();
            set_requests.push(SetSignalRequest {
                path,
                state: Some(state),
            });
            let request = tonic::Request::new(SetRequest { signals: set_requests  });

            let response = client.unwrap().set(request).await?.get_mut();
            ret.results.append(&mut response.results);
        }

        Ok(ret)
    }

    /// Subscribe to signal changes
    pub async fn subscribe(
        &mut self,
        path: String,
    ) -> Result<tonic::codec::Streaming<SubscribeResponse>> {
        info!("Subscribing to signal: {:?}", path);

        let client = self.get_target_client(path.clone());
        if client.is_none(){
            return Err(ClientError::NotFound(format!("client for {} not found", path)));
        }
        let client = client.unwrap();

        let request = tonic::Request::new(SubscribeRequest  { paths: [ path ].to_vec() });
        let response = client.subscribe(request).await?.into_inner();

        Ok(response)
    }

    /// Unsubscribe from signal changes
    pub async fn unsubscribe(&mut self, _: Vec<String>) -> Result<UnsubscribeResponse> {
        Err(ClientError::NotFound("Not Implemented yet".to_string()))
    }

    fn get_target_client<'a>(&'a mut self, path: String) -> Option<&'a mut SignalServiceClient<Channel>> {
        // TODO: fix unwrap
        let (_, result) = self.clients.iter_mut().find(|(prefix, client)| { path.starts_with(prefix) }).unwrap();
        Some(result)
    }

}
