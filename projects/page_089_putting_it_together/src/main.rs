#[cfg(test)]
mod should {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn succeed_logout_given_run() {
        let expected = "Logout view";
        let actual = block_on(logout());

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeed_login_given_run() {
        let expected = "Login view";
        let actual = block_on(login());

        assert_eq!(expected, actual);
    }
}

use actix_web::{web, App, HttpServer};

pub async fn logout() -> String {
    format!("Logout view")
}

pub async fn login() -> String {
    format!("Login view")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .route("/auth/login", web::get().to(login))
            .route("/auth/logout", web::get().to(logout));

        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
