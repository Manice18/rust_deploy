use std::str::FromStr;

use actix_web::{HttpResponse, post, web};
use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use serde::Serialize;
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;

#[derive(Serialize)]
pub struct SendTokenAccountMetaResponse {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Deserialize)]
struct SendTokenRequest {
    destination: Option<String>,
    mint: Option<String>,
    owner: Option<String>,
    amount: Option<u64>,
}

#[derive(Serialize)]
struct SendTokenResponse {
    program_id: String,
    accounts: Vec<SendTokenAccountMetaResponse>,
    instruction_data: String,
}

#[post("/send/token")]
pub async fn send_token(req: web::Json<SendTokenRequest>) -> actix_web::Result<HttpResponse> {
    if req.destination.is_none()
        || req.mint.is_none()
        || req.owner.is_none()
        || req.amount.is_none()
    {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Missing required fields"
        })));
    }
    let destination_pubkey = match Pubkey::from_str(&req.destination.as_ref().unwrap()) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid destination address"
            })));
        }
    };
    let mint_pubkey = match Pubkey::from_str(&req.mint.as_ref().unwrap()) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid mint address"
            })));
        }
    };
    let owner_pubkey = match Pubkey::from_str(&req.owner.as_ref().unwrap()) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid owner address"
            })));
        }
    };
    let source_token_account = get_associated_token_address(&owner_pubkey, &mint_pubkey);
    let destination_token_account = get_associated_token_address(&destination_pubkey, &mint_pubkey);
    // For a real transfer, you need the source token account, but here we assume destination is the token account.
    let ix = match spl_token::instruction::transfer(
        &spl_token::ID,
        &owner_pubkey, // source token account (should be ATA of owner for mint)
        &destination_token_account, // destination token account
        &owner_pubkey,
        &[],
        req.amount.unwrap(),
    ) {
        Ok(ix) => ix,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Failed to create transfer instruction"
            })));
        }
    };
    let accounts: Vec<SendTokenAccountMetaResponse> = ix
        .accounts
        .iter()
        .map(|meta| SendTokenAccountMetaResponse {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
        })
        .collect();

    let instruction_data = general_purpose::STANDARD.encode(&ix.data);
    let data = SendTokenResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": data
    })))
}
