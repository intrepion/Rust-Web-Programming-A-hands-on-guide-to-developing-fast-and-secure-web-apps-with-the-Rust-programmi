use super::path::Path;
use actix_web::web;

mod content_loader;
mod items;

pub fn app_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path {
        prefix: String::from("/"),
    };
    app.route(
        &base_path.define(String::from("")),
        web::get().to(items::items),
    );
}
