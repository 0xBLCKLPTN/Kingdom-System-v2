use actix_web::{put, get, post, Responder, HttpResponse};


#[get("/teachers")]
pub async fn get_schedule() -> impl Responder {
    HttpResponse::Ok().body("Hello World! GET")
}

#[post("/teacher")]
pub async fn create_schedule() -> impl Responder {
    HttpResponse::Ok().body("Hello World! POST")
}

#[put("/teacher")]
pub async fn edit_schedule() -> impl Responder {
    HttpResponse::Ok().body("Hello World! PUT")
}