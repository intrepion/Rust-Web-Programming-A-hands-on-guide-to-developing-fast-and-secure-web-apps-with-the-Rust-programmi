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

use actix_service::Service;
use actix_web::{App, HttpServer};
mod json_serialization;
mod processes;
mod state;
mod to_do;
mod views;

fn some_function() {}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    some_function();
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                if *&req.path().contains("/item/") {
                    match views::token::process_token(&req) {
                        Ok(_token) => println!("the token is passable"),
                        Err(message) => println!("token error: {}", message),
                    }
                }
                let fut = srv.call(req);
                async {
                    let result = fut.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory);

        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
