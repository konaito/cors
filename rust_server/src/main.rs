use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"message": "別オリジンAPI (Rust)"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on :8081");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
