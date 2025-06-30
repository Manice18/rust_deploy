use actix_web::{HttpResponse, post, web};
use base64;
use bs58;
use serde::Deserialize;
use serde::Serialize;

use crate::helpers::ApiResponse;

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
        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": false,
            "error": "Missing required fields"
        })));
    }
    // Decode pubkey
    let pubkey_bytes = match bs58::decode(&req.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid public key encoding"
            })));
        }
    };
    let pubkey = match solana_sdk::pubkey::Pubkey::try_from(pubkey_bytes.as_slice()) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid public key bytes"
            })));
        }
    };
    // Decode signature
    let signature_bytes = match base64::decode(&req.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid signature encoding"
            })));
        }
    };
    let signature = match solana_sdk::signature::Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid signature bytes"
            })));
        }
    };
    // Verify
    let valid = signature.verify(pubkey.as_ref(), req.message.as_bytes());
    let data = MessageSignResponse {
        valid,
        message: req.message.clone(),
        pubkey: req.pubkey.clone(),
    };
    let response = ApiResponse {
        success: true,
        data: data,
    };
    Ok(HttpResponse::Ok().json(response))
}
