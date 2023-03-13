use crate::client_api::{key_manager::keyManagerService, opa::opaService};
use crate::resource::Resources;
use crate::resources::directory_keymanager::KeyManager;
use tonic::transport::Server;

use clientApi::key_manager_service_server::KeyManagerServiceServer;
use clientApi::opa_service_server::OpaServiceServer;
use clientApi::resource_service_server::ResourceServiceServer;

use anyhow::*;

use super::resource::ResourceManager;
use std::sync::{Arc, Mutex};

pub mod clientApi {
    tonic::include_proto!("clientapi");
}

lazy_static::lazy_static! {
    pub static ref RESOURCE_MANAGER: Arc<Mutex<Resources>> = Arc::new(Mutex::new(Resources::default().expect("resource manager init")));
    pub static ref KEY_MANAGER: Arc<Mutex<KeyManager>> = Arc::new(Mutex::new(KeyManager::default().expect("key manager init")));
    // pub static ref OPA: Arc<ResourceManager> = Arc::new(ResourceManager::default().expect("resource manager init"));
}

pub async fn server(addr: &str) -> Result<()> {
    let addr = addr.parse()?;
    let opa_service = opaService::default();

    Server::builder()
        .add_service(ResourceServiceServer::new(ResourceManager::new(
            RESOURCE_MANAGER.clone(),
        )))
        .add_service(KeyManagerServiceServer::new(keyManagerService::new(
            KEY_MANAGER.clone(),
        )))
        .add_service(OpaServiceServer::new(opa_service))
        .serve(addr)
        .await?;

    Ok(())
}
