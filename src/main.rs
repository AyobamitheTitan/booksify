use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use api::{book::book::{add_book, find_book_by_id, get_books}, user::user::*};
use error::error::error::custom_json_error;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();
    let connect_options =
        ConnectOptions::new(std::env::var("DATABASE_URL").expect("DATABASE_URL is not defined"))
            .to_owned();

    let db = Database::connect(connect_options).await.unwrap();


    migration::Migrator::up(&db, None).await.unwrap();
    HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(custom_json_error))
            .service(web::scope("/api/auth").service(register).service(login))
            .service(web::scope("/api").service(add_book).service(find_book_by_id).service(get_books))
            .service(hello)
            
    })
    .bind(("127.0.0.1", 9872))?
    .run()
    .await
}
