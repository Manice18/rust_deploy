use actix_web::{post, web::Json};
use bs58;
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};
use std::io::Result;

use crate::helpers::ApiResponse;

#[derive(Serialize)]
struct KeypairData {
    pubkey: String,
    secret: String,
}

#[post("/keypair")]
pub async fn create_keypair() -> Result<Json<ApiResponse<KeypairData>>> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();
    let data = KeypairData { pubkey, secret };
    let response = ApiResponse {
        success: true,
        data,
    };
    Ok(Json(response))
}
