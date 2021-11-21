use actix_web::HttpRequest;
use serde_json::value::Value;
use serde_json::Map;

use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do;

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file("./state.json"); // 1

    let title: String = req.match_info().get("title").unwrap().to_string();
    let title_reference: String = title.clone(); // 2

    let item = to_do::to_do_factory(&String::from("pending"), &title).expect("create "); // 3
    process_input(item, "create".to_string(), &state); // 4

    return format!("{} created", title_reference) // 5
}
