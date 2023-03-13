use std::fs;

use crate::client_api::{
    resource_service_client::ResourceServiceClient, GetResourceRequest, SetResourceRequest,
};

pub async fn set_resource_cmd(path: &str, resource_id: &str, addr: &str) {
    let content = fs::read(path).unwrap();
    let request = SetResourceRequest {
        resource_id: resource_id.into(),
        content,
    };

    let mut client = ResourceServiceClient::connect(format!("http://{}", addr))
        .await
        .unwrap();

    client.set_resource(request).await.unwrap().into_inner();

    info!("set resource {resource_id} succeeded");
}

pub async fn get_resource_cmd(resource_id: &str, addr: &str) {
    let request = GetResourceRequest {
        resource_id: resource_id.into(),
    };

    let mut client = ResourceServiceClient::connect(format!("http://{}", addr))
        .await
        .unwrap();

    let response = client.get_resource(request).await.unwrap().into_inner();
    let content = String::from_utf8(response.content).unwrap();

    info!("get resource {resource_id} succeeded.");
    info!("resource content:\n{}", content);
}
