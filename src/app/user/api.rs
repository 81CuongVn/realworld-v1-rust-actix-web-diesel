use super::model::User;
use super::network::{request, response};
// use crate::schema::users;
use crate::utils::db::DbPool;
// use crate::AppState;
use actix_web::{get, post, put, web, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};

#[post("/login")]
pub async fn signin(
    pool: web::Data<DbPool>,
    form: web::Json<request::Signin>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let (user, token) =
        web::block(move || User::signin(&conn, &form.user.email, &form.user.password))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().json(e.to_string())
            })?;
    let res = response::Signin::from(user, token);
    Ok(HttpResponse::Ok().json(res))
}

#[post("")]
pub async fn signup(
    pool: web::Data<DbPool>,
    form: web::Json<request::Signup>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let (user, token) = web::block(move || {
        User::signup(
            &conn,
            &form.user.email,
            &form.user.username,
            &form.user.password,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().json(e.to_string())
    })?;

    let res = response::Signup::from(user, token);
    Ok(HttpResponse::Ok().json(res))
}

#[get("")]
pub async fn me() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("users me")
}

#[put("")]
pub async fn update() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("users update")
}