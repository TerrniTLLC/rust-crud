use actix_web::{get, patch, post, App, HttpResponse, HttpServer, Responder};

#[get("/noodles")]
async fn get_noodles() -> impl Responder {
    HttpResponse::Ok().body("Noodles avaliable !!")
}

#[post("/buy_noodle")]
async fn buy_noodle() -> impl Responder {
    HttpResponse::Ok().body("Buy a Noodle avaliable")
}

#[patch("/update_noodle/{uuid}")]
async fn update_noodle() -> impl Responder {
    HttpResponse::Ok().body("Update a Noodle avaliable")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_noodles)
            .service(buy_noodle)
            .service(update_noodle)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
