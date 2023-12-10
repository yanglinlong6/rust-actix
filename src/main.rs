mod models;
mod routes;
mod schema;
mod actions;

// Enable use diesel macro
#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::middleware::Logger;
// use diesel::prelude::*;
use diesel::r2d2::{ self, ConnectionManager };
use diesel::mysql::MysqlConnection;

use actix_web::{ post, Error, web, App, HttpResponse, HttpServer, Responder };
use routes::*;
// use diesel::prelude::*;
// use dotenv::dotenv;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection_pool() -> r2d2::Pool<ConnectionManager<MysqlConnection>> {
    // 从环境变量中读取数据库连接信息
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    // 创建连接管理器
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    // 创建连接池
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create database pool");

    pool
}

async fn aaa() -> impl Responder {
    println!("aaa");
    HttpResponse::Ok().body("aaa")
}

#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<models::NewUser>
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    println!("{}", new_user.username);
    let user = web
        ::block(move || {
            let conn = pool.get()?;
            actions::insert_new_user(new_user.into_inner(), &conn)
        }).await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    // dbg!(&user);
    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // 获取数据库连接池
    let pool: r2d2::Pool<ConnectionManager<MysqlConnection>> = establish_connection_pool();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // 将连接池作为应用程序数据共享
            .service(hello)
            // .service(rust)
            .service(index)
            .service(add_user)
            .service(name)
            .route("aaa", web::post().to(aaa))
            .wrap(Logger::default())
    })
        .bind("127.0.0.1:8080")?
        .run().await
}

// fn main() {
//     println!("Hello, world!");
// }
