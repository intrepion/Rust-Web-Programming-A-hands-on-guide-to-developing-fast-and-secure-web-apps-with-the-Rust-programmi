use actix_web::HttpResponse;

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            "<html>\
    <script>\
        localStorage.removeItem('user-token');\
        windows.location.replace(document.location.origin);\
    </script>\
</html>",
        )
}
