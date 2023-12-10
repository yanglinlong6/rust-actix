use actix_web::{ get, post, web, HttpResponse, Responder, Result };

use crate::models::*;

#[get("/")]
pub async fn hello() -> impl Responder {
    println!("hello");
    HttpResponse::Ok().body("Hello world!")
}

// extract `Info` using serde
#[post("/index")]
async fn index(user: web::Json<TbUser>) -> Result<String> {
    println!("index: {}", user.username);
    Ok(format!("Welcome {}!", user.username))
}

#[post("/name/{name}")]
async fn name(name: web::Path<String>) -> Result<impl Responder> {
    println!("{}", name);
    let obj = TbUser {
        id: 1,
        username: name.to_string(),
        password: "password value".to_owned(),
        email: "email value".to_owned(),
    };
    dbg!(&obj);
    println!("{:?}", obj);
    println!("{:#?}", obj);
    Ok(web::Json(obj))
}
