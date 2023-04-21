use crate::attestation_agent::rats_tls;
use crate::crypto::aes256_gcm;
use crate::resources;
use base64;
use serde_json::Value;

fn handle_version() -> Result<String, String> {
    let mut response = serde_json::Map::new();
    response.insert("status".to_string(), Value::String("OK".to_string()));
    response.insert("version".to_string(), Value::String("v1".to_string()));

    Ok(Value::Object(response).to_string())
}

fn handle_decrypt(request: &Value) -> Result<String, String> {
    let blobs = match request["blobs"].as_array() {
        Some(blobs) => blobs,
        None => return Err("decrypt parameters error".to_string()),
    };

    let mut response = serde_json::Map::new();
    response.insert("status".to_string(), Value::String("OK".to_string()));
    let mut data = serde_json::Map::new();

    for blob in blobs {
        if blob["algorithm"] != "AES"
            || blob["key_length"] != 256
            || blob["encrypted_data"].is_null()
            || blob["iv"].is_null()
        {
            return Err("parameters error".to_string());
        }

        match resources::directory_key_manager::get_key(&String::from(
            blob["kid"].as_str().unwrap(),
        ))
        .map_err(|_| format!("kid: {}'s key not found", blob["kid"].to_string()))
        .and_then(|key| {
            let iv = base64::decode(blob["iv"].as_str().unwrap()).unwrap();
            let encrypted_data = base64::decode(blob["encrypted_data"].as_str().unwrap()).unwrap();
            aes256_gcm::decrypt(&encrypted_data, key.as_slice(), &iv)
                .map_err(|_| "decryption failed".to_string())
                .and_then(|decrypted_data| Ok(decrypted_data))
        }) {
            Ok(decrypted_data) => data.insert(
                blob["encrypted_data"].as_str().unwrap().to_string(),
                Value::String(base64::encode(decrypted_data)),
            ),
            Err(e) => return Err(e),
        };
    }
    response.insert("data".to_string(), Value::Object(data));

    Ok(Value::Object(response).to_string())
}

fn handle_getKek(request: &Value) -> Result<String, String> {
    let blobs = match request["kids"].as_array() {
        Some(blobs) => blobs,
        None => return Err("get KEK parameters error".to_string()),
    };

    let mut response = serde_json::Map::new();
    response.insert("status".to_string(), Value::String("OK".to_string()));
    let mut data = serde_json::Map::new();

    for index in 0..blobs.len() {
        let kid = blobs[index].as_str().unwrap();
        match resources::directory_key_manager::get_key(&String::from(kid))
            .map_err(|_| format!("kid: {}'s key not found", kid))
            .and_then(|key| Ok(key))
        {
            Ok(key) => data.insert(String::from(kid), Value::String(base64::encode(key))),
            Err(e) => return Err(e),
        };
    }
    response.insert("data".to_string(), Value::Object(data));

    Ok(Value::Object(response).to_string())
}

fn handle_echo(request: &Value) -> Result<String, String> {
    let data = match request["data"].as_str() {
        Some(data) => data,
        None => return Err("Echo parameters error".to_string()),
    };

    Ok(data.to_owned())
}

fn handle_get_policy() -> Result<String, String> {
    resources::image::export_base64(resources::image::POLICY)
        .map_err(|e| format!("Can't fetch policy.json file, error:{}", e))
}

fn handle_get_sigstore_config() -> Result<String, String> {
    resources::image::export_base64(resources::image::SIGSTORE)
        .map_err(|e| format!("Can't fetch sigstore.yaml file, error:{}", e))
}

fn handle_get_cosign_key() -> Result<String, String> {
    resources::image::export_base64(resources::image::COSIGN)
        .map_err(|e| format!("Can't fetch cosign key file, error:{}", e))
}

fn handle_get_credential() -> Result<String, String> {
    resources::image::export_base64(resources::image::CREDENTIAL)
        .map_err(|e| format!("Can't fetch cosign key file, error:{}", e))
}

fn handle_get_gpg_keyring() -> Result<String, String> {
    resources::gpg::export_base64()
        .map_err(|e| format!("Can't fetch gpg keyring file, error:{}", e))
}

fn handle_get_resource_info(request: &Value) -> Result<String, String> {
    let mut response = serde_json::Map::new();
    response.insert("status".to_string(), Value::String("OK".to_string()));

    match request["name"].as_str().unwrap() {
        "GPG Keyring" => resources::gpg::size_base64(),
        "Policy" => resources::image::size_base64(resources::image::POLICY),
        "Sigstore Config" => resources::image::size_base64(resources::image::SIGSTORE),
        "Cosign Key" => resources::image::size_base64(resources::image::COSIGN),
        "Credential" => resources::image::size_base64(resources::image::CREDENTIAL),
        _ => Err("file name error".to_string()),
    }
    .map_err(|e| e)
    .and_then(|size| {
        let mut info = serde_json::Map::new();
        info.insert("base64size".to_string(), Value::String(size.to_string()));
        response.insert("data".to_string(), Value::Object(info));
        Ok(Value::Object(response).to_string())
    })
}

fn error_message(e: String) -> Result<String, ()> {
    let msg = serde_json::json!({
        "status": "Fail",
        "data": {},
        "error": e
    })
    .to_string();
    Ok(msg)
}

fn error_message2(e: String) -> Result<String, ()> {
    let msg = serde_json::json!({
        "status": "Fail",
        "error": e
    })
    .to_string();
    Ok(msg)
}

pub fn handle(request: &[u8]) -> Result<(String, u8), String> {
    let parsed_request: Value = match serde_json::from_slice(request) {
        Ok(r) => r,
        Err(_) => return Err("Parse request failed".to_string()),
    };
    info!("Request: {:?}", parsed_request);

    let response = match parsed_request["command"].as_str().unwrap() {
        "version" => {
            let response = handle_version().unwrap();
            Ok((response, rats_tls::ACTION_NONE))
        }
        "Decrypt" => {
            let response =
                handle_decrypt(&parsed_request).unwrap_or_else(|e| error_message(e).unwrap());
            Ok((response, rats_tls::ACTION_NONE))
        }
        "Get KEK" => {
            let response =
                handle_getKek(&parsed_request).unwrap_or_else(|e| error_message(e).unwrap());
            Ok((response, rats_tls::ACTION_NONE))
        }
        "echo" => {
            let response = handle_echo(&parsed_request).unwrap_or_else(|e| e);
            Ok((response, rats_tls::ACTION_DISCONNECT))
        }
        "Get Policy" => {
            let response =
                handle_get_policy().unwrap_or_else(|e| base64::encode(error_message2(e).unwrap()));
            Ok((response, rats_tls::ACTION_NONE))
        }
        "Get Sigstore Config" => {
            let response = handle_get_sigstore_config()
                .unwrap_or_else(|e| base64::encode(error_message2(e).unwrap()));
            Ok((response, rats_tls::ACTION_NONE))
        }
        "Get GPG Keyring" => {
            let response = handle_get_gpg_keyring()
                .unwrap_or_else(|e| base64::encode(error_message2(e).unwrap()));
            Ok((response, rats_tls::ACTION_NONE))
        }
        "Get Resource Info" => {
            let response = handle_get_resource_info(&parsed_request)
                .unwrap_or_else(|e| error_message2(e).unwrap());
            Ok((response, rats_tls::ACTION_NONE))
        }
        "Get Cosign Key" => {
            let response = handle_get_cosign_key()
                .unwrap_or_else(|e| base64::encode(error_message2(e).unwrap()));
            Ok((response, rats_tls::ACTION_NONE))
        }
        "Get Credential" => {
            let response = handle_get_credential()
                .unwrap_or_else(|e| base64::encode(error_message2(e).unwrap()));
            Ok((response, rats_tls::ACTION_NONE))
        }
        _ => Err("Command error".to_string()),
    };

    response
}
