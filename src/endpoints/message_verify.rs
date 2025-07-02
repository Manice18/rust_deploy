use actix_web::{HttpResponse, post, web};
use bs58;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
struct MessageVerifyRequest {
    message: String,
    signature: String,
    pubkey: String,
}

#[derive(Serialize)]
struct MessageSignResponse {
    valid: bool,
    message: String,
    pubkey: String,
}

#[post("/message/verify")]
pub async fn message_verify(
    req: web::Json<MessageVerifyRequest>,
) -> actix_web::Result<HttpResponse> {
    // Check for missing fields
    if req.message.is_empty() || req.signature.is_empty() || req.pubkey.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Missing required fields"
        })));
    }
    // Decode pubkey (base58)
    let pubkey_bytes = match bs58::decode(&req.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid public key encoding"
            })));
        }
    };
    let pubkey = match ed25519_dalek::PublicKey::from_bytes(&pubkey_bytes) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid public key bytes"
            })));
        }
    };
    // Decode signature (base58)
    let signature_bytes = match bs58::decode(&req.signature).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid signature encoding"
            })));
        }
    };
    let signature = match ed25519_dalek::Signature::from_bytes(&signature_bytes) {
        Ok(sig) => sig,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid signature bytes"
            })));
        }
    };
    // Verify
    let valid = pubkey
        .verify_strict(req.message.as_bytes(), &signature)
        .is_ok();
    let data = MessageSignResponse {
        valid,
        message: req.message.clone(),
        pubkey: req.pubkey.clone(),
    };
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": data
    })))
}
