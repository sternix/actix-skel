use crate::error::Error;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct LoginResult {
    status: String,
    message: String,
}

async fn login(
    _pool: web::Data<deadpool_postgres::Pool>,
    body: web::Bytes,
    id: Identity,
) -> Result<HttpResponse, Error> {
    let Login { username, password } = serde_json::from_slice::<Login>(&body)?;
    if username == "username" && password == "password" {
        id.remember(username.to_owned());
        let status = LoginResult {
            status: "ok".into(),
            message: "Login Başarılı".into(),
        };

        Ok(HttpResponse::Ok().json(status))
    } else {
        let status = LoginResult {
            status: "err".into(),
            message: "Kullanıcı Bilgileri Hatalı !".into(),
        };
        Ok(HttpResponse::Ok().json(status))
    }
}

async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().json("Logout Başarılı")
}

async fn test(id: Identity) -> HttpResponse {
    if id.identity().is_some() {
        HttpResponse::Ok().json("Login Olunmuş, ***Başarılı*** ")
    } else {
        HttpResponse::Ok().json("Login olunmamış, !!! Yetkisiz !!! ")
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/auth/login").route(web::post().to(login)));
    cfg.service(web::resource("/auth/logout").route(web::get().to(logout)));
    cfg.service(web::resource("/auth/test").route(web::get().to(test)));
}

// test için
// curl --cookie-jar cookies.txt  --cookie cookies.txt -i -X GET http://localhost:8080/auth/test
// curl --cookie-jar cookies.txt  --cookie cookies.txt -i -X POST -H 'Content-Type: application/json' -d '{"username":"username","password":"password"}' http://localhost:8080/auth/login
// curl --cookie-jar cookies.txt  --cookie cookies.txt -i -X GET http://localhost:8080/auth/test
// curl --cookie-jar cookies.txt  --cookie cookies.txt -i -X GET http://localhost:8080/auth/logout
// curl --cookie-jar cookies.txt  --cookie cookies.txt -i -X GET http://localhost:8080/auth/test
