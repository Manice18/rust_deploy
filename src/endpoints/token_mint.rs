use std::str::FromStr;

use actix_web::{HttpResponse, post, web};
use base64;
use serde::Deserialize;
use serde::Serialize;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use spl_token::instruction::mint_to;

use crate::helpers::AccountMetaResponse;

#[derive(Deserialize)]
struct TokenMintRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

#[derive(Serialize)]
struct TokenMintResponse {
    program_id: String,
    accounts: Vec<AccountMetaResponse>,
    instruction_data: String,
}

#[post("/token/mint")]
pub async fn token_mint(req: web::Json<TokenMintRequest>) -> actix_web::Result<HttpResponse> {
    let mint_pubkey = match Pubkey::from_str(&req.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid mint pubkey"
            })));
        }
    };
    let destination_pubkey = match Pubkey::from_str(&req.destination) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid destination pubkey"
            })));
        }
    };
    let authority_pubkey = match Pubkey::from_str(&req.authority) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid authority pubkey"
            })));
        }
    };
    let amount = req.amount;
    let ix = match mint_to(
        &spl_token::ID,
        &mint_pubkey,
        &destination_pubkey,
        &authority_pubkey,
        &[&authority_pubkey],
        amount,
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Failed to create instruction"
            })));
        }
    };

    let accounts: Vec<AccountMetaResponse> = ix
        .accounts
        .iter()
        .map(|meta| AccountMetaResponse {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect();

    let instruction_data = base64::encode(&ix.data);

    let data = TokenMintResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };
    let response = serde_json::json!({
        "success": true,
        "data": data
    });
    Ok(HttpResponse::Ok().json(response))
}
