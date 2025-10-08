use anyhow::Error;
use async_trait::async_trait;
use libp2p::{
    dcutr, gossipsub, identify, identity::Keypair, kad::{self, store::MemoryStore}, mdns, ping, relay, request_response::cbor, swarm::NetworkBehaviour, Swarm
};
use thiserror::Error;
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;

use crate::app::Service;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileChunkRequest {
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileChunkResponse {
    pub data: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum P2pNetworkError {
   
}

#[derive(NetworkBehaviour)]
pub struct P2pNetworkBehaviour {
    ping: ping::Behaviour,
    identify: identify::Behaviour,
    mdns: mdns::Behaviour<mdns::tokio::Tokio>,
    kademlia: kad::Behaviour<MemoryStore>,
    relay_server: relay::Behaviour,
    relay_client: relay::client::Behaviour,
    gossipsub: gossipsub::Behaviour,
    dcutr: dcutr::Behaviour,
    file_download: cbor::Behaviour<FileChunkRequest, FileChunkResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct P2pServiceConfig {
    
}

pub struct P2pService {

}

impl P2pService {
    pub fn new() -> Self {
        Self {}
    }

    async fn keypair(&self)

    fn swarm() -> Result<Swarm<P2pNetworkBehaviour>, P2pNetworkError> {
        
        let keypair = Keypair::generate_ed25519();
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
    }
}

#[async_trait]
impl Service for P2pService {
    async fn start(&self, cancel_token: CancellationToken) -> Result<(), Error> {
        // TODO: Implement
        Ok(())
    }
}
