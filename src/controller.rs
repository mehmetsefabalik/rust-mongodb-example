use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct User {
    name: String,
}

pub async fn index(app_data: web::Data<crate::AppState>, user: web::Query<User>) -> impl Responder {
    let result = web::block(move || app_data.service_container.user.create(&user.name)).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get(app_data: web::Data<crate::AppState>) -> impl Responder {
    let result = web::block(move || app_data.service_container.user.get()).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
