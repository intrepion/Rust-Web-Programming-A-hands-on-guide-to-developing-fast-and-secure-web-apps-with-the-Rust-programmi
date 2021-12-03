#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_some_function_given_run() {
        let expected = ();
        let actual = some_function();

        assert_eq!(expected, actual);
    }
}

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod auth;
mod database;
mod json_serialization;
mod models;
mod processes;
mod schema;
mod state;
mod to_do;
mod views;
use actix_service::Service;
use actix_web::{App, HttpResponse, HttpServer};
use futures::future::{ok, Either};

fn some_function() {}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    some_function();
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                let passed: bool;

                if *&req.path().contains("/item/") {
                    match auth::process_token(&req) {
                        Ok(_token) => {
                            passed = true;
                        }
                        Err(_message) => {
                            passed = false;
                        }
                    }
                } else {
                    passed = true;
                }

                let end_result = match passed {
                    true => Either::Left(srv.call(req)),
                    false => Either::Right(ok(
                        req.into_response(HttpResponse::Unauthorized().finish().into_body())
                    )),
                };

                end_result
            })
            .configure(views::views_factory);

        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
