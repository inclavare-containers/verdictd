use crate::attestation_agent::rats_tls;
use crate::client_api::api::{KEY_MANAGER, RESOURCE_MANAGER};
use crate::crypto::aes256_gcm;
use anyhow::*;
use base64;
use serde_json::Value;

fn handle_version() -> Result<String> {
    let mut response = serde_json::Map::new();
    response.insert("status".to_string(), Value::String("OK".to_string()));
    response.insert("version".to_string(), Value::String("v1".to_string()));

    Ok(Value::Object(response).to_string())
}

fn handle_decrypt(request: &Value) -> Result<String> {
    let blobs = match request["blobs"].as_array() {
        Some(blobs) => blobs,
        None => bail!("decrypt parameters error"),
    };

    let mut response = serde_json::Map::new();
    response.insert("status".to_string(), Value::String("OK".to_string()));
    let mut data = serde_json::Map::new();

    for blob in blobs {
        if blob["algorithm"] != "A256GCM"
            || blob["key_length"] != 256
            || blob["encrypted_data"].is_null()
            || blob["iv"].is_null()
        {
            bail!("parameters error");
        }

        match KEY_MANAGER
            .clone()
            .lock()
            .unwrap()
            .get_key(&String::from(blob["kid"].as_str().unwrap()))
            .map_err(|_| format!("kid: {}'s key not found", blob["kid"].to_string()))
            .and_then(|key| {
                let iv = base64::decode(blob["iv"].as_str().unwrap()).unwrap();
                let encrypted_data =
                    base64::decode(blob["encrypted_data"].as_str().unwrap()).unwrap();
                aes256_gcm::decrypt(&encrypted_data, key.as_slice(), &iv)
                    .map_err(|_| "decryption failed".to_string())
                    .and_then(|decrypted_data| std::result::Result::Ok(decrypted_data))
            }) {
            std::result::Result::Ok(decrypted_data) => data.insert(
                blob["encrypted_data"].as_str().unwrap().to_string(),
                Value::String(base64::encode(decrypted_data)),
            ),
            Err(e) => bail!(e),
        };
    }
    response.insert("data".to_string(), Value::Object(data));

    Ok(Value::Object(response).to_string())
}

fn handle_getKek(request: &Value) -> Result<String> {
    let blobs = match request["kids"].as_array() {
        Some(blobs) => blobs,
        None => bail!("get KEK parameters error"),
    };

    let mut response = serde_json::Map::new();
    response.insert("status".to_string(), Value::String("OK".to_string()));
    let mut data = serde_json::Map::new();

    for index in 0..blobs.len() {
        let kid = blobs[index].as_str().unwrap();
        match KEY_MANAGER
            .clone()
            .lock()
            .unwrap()
            .get_key(&String::from(kid))
            .map_err(|_| format!("kid: {}'s key not found", kid))
            .and_then(|key| std::result::Result::Ok(key))
        {
            std::result::Result::Ok(key) => {
                data.insert(String::from(kid), Value::String(base64::encode(key)))
            }
            Err(e) => bail!(e),
        };
    }
    response.insert("data".to_string(), Value::Object(data));

    Ok(Value::Object(response).to_string())
}

fn handle_echo(request: &Value) -> Result<String> {
    let data = match request["data"].as_str() {
        Some(data) => data,
        None => bail!("Echo parameters error"),
    };

    Ok(data.to_owned())
}

fn handle_get_resource(request: &Value) -> Result<String> {
    let resource_path = request["optional"]["resource_path"].as_str().unwrap();

    let resource = RESOURCE_MANAGER
        .clone()
        .lock()
        .unwrap()
        .get_resource(resource_path)?;
    let base64_encoded = base64::encode(resource);
    Ok(base64_encoded)
}

fn handle_get_resource_info(request: &Value) -> Result<String> {
    let mut response = serde_json::Map::new();
    response.insert("status".to_string(), Value::String("OK".to_string()));

    let resource_path = request["name"].as_str().unwrap();
    let size = RESOURCE_MANAGER
        .clone()
        .lock()
        .unwrap()
        .get_resource_info(resource_path)
        .context("read from fs")?;
    let mut info = serde_json::Map::new();
    info.insert("base64size".to_string(), Value::String(size.to_string()));
    response.insert("data".to_string(), Value::Object(info));
    Ok(Value::Object(response).to_string())
}

fn error_message(e: String) -> String {
    serde_json::json!({
        "status": "Fail",
        "data": {},
        "error": e
    })
    .to_string()
}

fn error_message2(e: String) -> String {
    serde_json::json!({
        "status": "Fail",
        "error": e
    })
    .to_string()
}

pub fn handle(request: &[u8]) -> Result<(String, u8)> {
    let parsed_request: Value = match serde_json::from_slice(request) {
        std::result::Result::Ok(r) => r,
        Err(_) => bail!("Parse request failed"),
    };
    info!("Request: {:?}", parsed_request);

    let response = match parsed_request["command"].as_str().unwrap() {
        "version" => {
            let response = handle_version().unwrap();
            Ok((response, rats_tls::ACTION_NONE))
        }
        "Decrypt" => {
            let response =
                handle_decrypt(&parsed_request).unwrap_or_else(|e| error_message(e.to_string()));
            Ok((response, rats_tls::ACTION_NONE))
        }
        "Get KEK" => {
            let response =
                handle_getKek(&parsed_request).unwrap_or_else(|e| error_message(e.to_string()));
            Ok((response, rats_tls::ACTION_NONE))
        }
        "echo" => {
            let response = handle_echo(&parsed_request).unwrap_or_else(|e| e.to_string());
            Ok((response, rats_tls::ACTION_DISCONNECT))
        }
        "Get Resource" => {
            let response = handle_get_resource(&parsed_request)
                .unwrap_or_else(|e| base64::encode(error_message2(e.to_string())));
            Ok((response, rats_tls::ACTION_NONE))
        }
        "Get Resource Info" => {
            let response = handle_get_resource_info(&parsed_request)
                .unwrap_or_else(|e| error_message2(e.to_string()));
            Ok((response, rats_tls::ACTION_NONE))
        }
        _ => bail!("Command error"),
    };

    response
}
