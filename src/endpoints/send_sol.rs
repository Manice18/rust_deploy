use std::str::FromStr;

use actix_web::{HttpResponse, post, web};
use bs58;
use serde::Deserialize;
use serde::Serialize;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;

#[derive(Deserialize)]
struct SendSolRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Serialize)]
struct SendSolResponse {
    program_id: String,
    accounts: Vec<String>,
    instruction_data: String,
}

#[post("/send/sol")]
pub async fn send_sol(req: web::Json<SendSolRequest>) -> actix_web::Result<HttpResponse> {
    if req.from.is_empty() || req.to.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Missing required fields"
        })));
    }
    let from_pubkey = match Pubkey::from_str(&req.from) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid sender public key"
            })));
        }
    };
    let to_pubkey = match Pubkey::from_str(&req.to) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid to address"
            })));
        }
    };
    if req.lamports == 0 {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Amount must be greater than 0"
        })));
    }
    let lamports = req.lamports;
    let ix = system_instruction::transfer(&from_pubkey, &to_pubkey, lamports);
    let accounts: Vec<String> = ix
        .accounts
        .iter()
        .map(|meta| meta.pubkey.to_string())
        .collect();
    let instruction_data = bs58::encode(&ix.data).into_string();
    let data = SendSolResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data,
    };
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": data
    })))
}
