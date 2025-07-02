use std::str::FromStr;

use actix_web::{HttpResponse, post, web};
use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use serde::Serialize;
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::mint_to;

use crate::helpers::AccountMetaResponse;

#[derive(Deserialize)]
struct TokenMintRequest {
    mint: Option<String>,
    destination: Option<String>,
    authority: Option<String>,
    amount: Option<u64>,
}

#[derive(Serialize)]
struct TokenMintResponse {
    program_id: String,
    accounts: Vec<AccountMetaResponse>,
    instruction_data: String,
}

#[post("/token/mint")]
pub async fn token_mint(req: web::Json<TokenMintRequest>) -> actix_web::Result<HttpResponse> {
    if req.mint.is_none()
        || req.destination.is_none()
        || req.authority.is_none()
        || req.amount.is_none()
    {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Missing required fields"
        })));
    }
    let mint_pubkey = match Pubkey::from_str(&req.mint.as_ref().unwrap()) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid mint pubkey"
            })));
        }
    };
    let destination_owner_pubkey = match Pubkey::from_str(&req.destination.as_ref().unwrap()) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid destination pubkey"
            })));
        }
    };
    let destination_token_account =
        get_associated_token_address(&destination_owner_pubkey, &mint_pubkey);
    let authority_pubkey = match Pubkey::from_str(&req.authority.as_ref().unwrap()) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid authority pubkey"
            })));
        }
    };
    let amount = req.amount;
    let ix = match mint_to(
        &spl_token::ID,
        &mint_pubkey,
        &destination_token_account,
        &authority_pubkey,
        &[],
        amount.unwrap(),
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
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

    let instruction_data = general_purpose::STANDARD.encode(&ix.data);

    let data = TokenMintResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": data
    })))
}
