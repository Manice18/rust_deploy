use actix_web::{HttpResponse, post};
use bs58;
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};

#[derive(Serialize)]
struct KeypairData {
    pubkey: String,
    secret: String,
}

#[post("/keypair")]
pub async fn create_keypair() -> actix_web::Result<HttpResponse> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();
    let data = KeypairData { pubkey, secret };
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": data
    })))
}
