use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::EntityTrait;

use crate::models::prelude::User;
use crate::{app_data::AppData, templates};

#[get("/")]
pub async fn index(data: web::Data<AppData>) -> impl Responder {
    let conn = &data.db;

    let user_result = User::find_by_id(1).one(conn).await;
    if let Err(err) = user_result {
        log::error!("{}", err);
        return HttpResponse::InternalServerError().finish();
    }

    let content = match user_result.unwrap() {
        Some(user) => user.email,
        None => "User not found!".to_string(),
    };

    let template = templates::IndexTemplate {
        title: "test".to_string(),
        content,
    };

    HttpResponse::Ok().body(template.to_string())
}
