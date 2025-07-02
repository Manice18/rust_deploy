use actix_web::{HttpResponse, post, web};
use bs58;
use serde::Deserialize;
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};

#[derive(Deserialize)]
struct MessageSignRequest {
    message: String,
    secret: String,
}

#[derive(Serialize)]
struct MessageSignResponse {
    signature: String,
    pubkey: String,
    message: String,
}

#[post("/message/sign")]
pub async fn message_sign(req: web::Json<MessageSignRequest>) -> actix_web::Result<HttpResponse> {
    let secret_bytes = match bs58::decode(&req.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid secret key encoding"
            })));
        }
    };
    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Missing required fields"
            })));
        }
    };
    if req.message.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Missing required fields"
        })));
    }
    let message_bytes = req.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);
    let signature_b58 = bs58::encode(signature.as_ref()).into_string();
    let pubkey_b58 = bs58::encode(keypair.pubkey().to_bytes()).into_string();
    let data = MessageSignResponse {
        signature: signature_b58,
        pubkey: pubkey_b58,
        message: req.message.clone(),
    };
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": data
    })))
}
