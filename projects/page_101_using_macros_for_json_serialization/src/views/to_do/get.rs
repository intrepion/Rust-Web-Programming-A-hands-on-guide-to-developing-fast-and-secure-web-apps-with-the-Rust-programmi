use actix_web::{web, Responder};
use crate::state::read_file;
use serde_json::value::Value;
use serde_json::Map;

pub async fn get() -> impl Responder {
    let state: Map<String, Value> = read_file("./state.json");

    return web::Json(state);
}
