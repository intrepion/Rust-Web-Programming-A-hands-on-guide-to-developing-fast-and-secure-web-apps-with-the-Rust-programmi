use super::utils::return_state;
use crate::auth::jwt::JwtToken;
use actix_web::{HttpRequest, Responder};

pub async fn get(req: HttpRequest) -> impl Responder {
    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();

    return return_state(&token.user_id);
}
