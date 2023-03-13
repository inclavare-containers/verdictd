use std::sync::{Arc, Mutex};

use crate::client_api::api;
use crate::resource::Resources;

use anyhow::*;
use tonic::{Request, Response, Status};

use api::clientApi::resource_service_server::ResourceService;

use super::api::clientApi::{
    GetResourceRequest, GetResourceResponse, SetResourceRequest, SetResourceResponse,
};

#[derive(Debug)]
pub struct ResourceManager {
    resource: Arc<Mutex<Resources>>,
}

impl ResourceManager {
    pub fn new(inner: Arc<Mutex<Resources>>) -> Self {
        Self { resource: inner }
    }
}

#[tonic::async_trait]
impl ResourceService for ResourceManager {
    async fn get_resource(
        &self,
        request: Request<GetResourceRequest>,
    ) -> Result<Response<GetResourceResponse>, Status> {
        let resource_id = request.into_inner().resource_id;
        let resource = self
            .resource
            .lock()
            .unwrap()
            .get_resource(&resource_id)
            .map_err(|e| Status::aborted(format!("resource get failed {}", e)))?;

        let res = GetResourceResponse { content: resource };
        std::result::Result::Ok(Response::new(res))
    }

    async fn set_resource(
        &self,
        request: Request<SetResourceRequest>,
    ) -> Result<Response<SetResourceResponse>, Status> {
        let inner = request.into_inner();
        let resource_id = inner.resource_id;
        let resource = inner.content;

        self.resource
            .lock()
            .unwrap()
            .set_resource(&resource_id, &resource)
            .map_err(|e| Status::aborted(format!("resource set failed {}", e)))?;

        let res = SetResourceResponse {};
        std::result::Result::Ok(Response::new(res))
    }
}
