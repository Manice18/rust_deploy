use std::str::FromStr;

use actix_web::{HttpResponse, post, web};
use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use serde::Serialize;
use solana_program::pubkey::Pubkey;
use spl_token::instruction::initialize_mint;

use crate::helpers::AccountMetaResponse;

#[derive(Deserialize)]
struct TokenCreateRequest {
    #[serde(rename = "mintAuthority")]
    mint_authority: Option<String>,
    mint: Option<String>,
    decimals: Option<u8>,
}

#[derive(Serialize)]
struct TokenCreateResponse {
    program_id: String,
    accounts: Vec<AccountMetaResponse>,
    instruction_data: String,
}

#[post("/token/create")]
pub async fn token_create(req: web::Json<TokenCreateRequest>) -> actix_web::Result<HttpResponse> {
    if req.mint.is_none() || req.mint_authority.is_none() || req.decimals.is_none() {
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
    let mint_authority_pubkey = match Pubkey::from_str(&req.mint_authority.as_ref().unwrap()) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid mint authority pubkey"
            })));
        }
    };
    let freeze_authority = None;
    let decimals = req.decimals;

    let ix = match initialize_mint(
        &spl_token::ID,
        &mint_pubkey,
        &mint_authority_pubkey,
        freeze_authority.as_ref(),
        decimals.unwrap(),
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

    let instruction_data = general_purpose::STANDARD.encode(ix.data);

    let data = TokenCreateResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": data
    })))
}
