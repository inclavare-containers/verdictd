use std::sync::{Arc, Mutex};

use crate::resources::directory_keymanager::KeyManager;
use anyhow::*;
use rand::*;
use tonic::{Request, Response, Status};

use super::api::clientApi::{
    key_manager_service_server::KeyManagerService, CreateKeyRequest, CreateKeyResponse,
    DeleteKeyRequest, DeleteKeyResponse, GetKeyRequest, GetKeyResponse,
};

// #[derive(Debug)]
pub struct keyManagerService {
    key_manager: Arc<Mutex<KeyManager>>,
}

impl keyManagerService {
    pub fn new(inner: Arc<Mutex<KeyManager>>) -> Self {
        Self { key_manager: inner }
    }
}

#[tonic::async_trait]
impl KeyManagerService for keyManagerService {
    async fn create_key(
        &self,
        request: Request<CreateKeyRequest>,
    ) -> Result<Response<CreateKeyResponse>, Status> {
        let req = request.into_inner();
        let kid = req.keyid;
        // generate a new key file with a new random key
        let mut key: [u8; 32] = [0; 32];
        rand::rngs::OsRng.fill_bytes(&mut key);
        self.key_manager
            .lock()
            .unwrap()
            .set_key(&kid, &key)
            .map_err(|e| Status::aborted(format!("create key failed {}", e)))?;

        std::result::Result::Ok(Response::new(CreateKeyResponse {}))
    }

    async fn get_key(
        &self,
        request: Request<GetKeyRequest>,
    ) -> Result<Response<GetKeyResponse>, Status> {
        let kid = request.into_inner().keyid;
        info!("kid: {}", kid);

        let key = self
            .key_manager
            .lock()
            .unwrap()
            .get_key(&kid)
            .map_err(|e| Status::aborted(format!("get key failed {}", e)))?;
        let res = GetKeyResponse { key };

        std::result::Result::Ok(Response::new(res))
    }

    async fn delete_key(
        &self,
        _request: Request<DeleteKeyRequest>,
    ) -> Result<Response<DeleteKeyResponse>, Status> {
        std::result::Result::Err(Status::aborted("Not Implemented"))
    }
}
