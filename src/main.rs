use std::io::Result;

use actix_web::HttpServer;

use crate::endpoints::keypair::create_keypair;
use crate::endpoints::message_sign::message_sign;
use crate::endpoints::message_verify::message_verify;
use crate::endpoints::send_sol::send_sol;
use crate::endpoints::send_token::send_token;
use crate::endpoints::token_create::token_create;
use crate::endpoints::token_mint::token_mint;

mod endpoints;
mod helpers;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(move || {
        actix_web::App::new()
            .service(create_keypair)
            .service(token_create)
            .service(token_mint)
            .service(message_sign)
            .service(message_verify)
            .service(send_sol)
            .service(send_token)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
