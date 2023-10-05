use actix_web::{put, get, post, Responder, HttpResponse};


#[get("/schedule")]
pub async fn get_schedule() -> impl Responder {
    HttpResponse::Ok().body("Hello World! GET")
}

#[post("/schedule")]
pub async fn create_schedule() -> impl Responder {
    HttpResponse::Ok().body("Hello World! POST")
}

#[put("/schedule")]
pub async fn edit_schedule() -> impl Responder {
    HttpResponse::Ok().body("Hello World! PUT")
}