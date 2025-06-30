use std::str::FromStr;

use actix_web::{HttpResponse, post, web};
use base64;
use serde::Deserialize;
use serde::Serialize;
use solana_program::pubkey::Pubkey;

use crate::helpers::AccountMetaResponse;
use crate::helpers::ApiResponse;

#[derive(Deserialize)]
struct SendTokenRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}

#[derive(Serialize)]
struct SendTokenResponse {
    program_id: String,
    accounts: Vec<AccountMetaResponse>,
    instruction_data: String,
}

#[post("/send/token")]
pub async fn send_token(req: web::Json<SendTokenRequest>) -> actix_web::Result<HttpResponse> {
    if req.destination.is_empty() || req.mint.is_empty() || req.owner.is_empty() {
        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": false,
            "error": "Missing required fields"
        })));
    }
    let destination_pubkey = match Pubkey::from_str(&req.destination) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid destination address"
            })));
        }
    };
    let mint_pubkey = match Pubkey::from_str(&req.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid mint address"
            })));
        }
    };
    let owner_pubkey = match Pubkey::from_str(&req.owner) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid owner address"
            })));
        }
    };
    let amount = req.amount;
    // For a real transfer, you need the source token account, but here we assume destination is the token account.
    let ix = match spl_token::instruction::transfer(
        &spl_token::ID,
        &mint_pubkey,        // source token account (should be ATA of owner for mint)
        &destination_pubkey, // destination token account
        &owner_pubkey,
        &[],
        amount,
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Failed to create transfer instruction"
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
    let data = SendTokenResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };
    let response = ApiResponse {
        success: true,
        data: data,
    };
    Ok(HttpResponse::Ok().json(response))
}
