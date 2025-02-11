pub mod book {
    use actix_web::{get, post, web, HttpResponse};
    use schema::book::book::NewBook;
    use serde_json::json;
    use service::{mutation::book::book::add_book as add_new_book, query::book::book::{find_book, get_books as get_all_books}};
    use uuid::Uuid;

    #[post("/book")]
    pub async fn add_book(payload: web::Json<NewBook>) -> HttpResponse {
        let book = add_new_book(payload.0).await;
        match book {
            Ok(bk) => HttpResponse::Ok().json(json!({
                "message":"New book added",
                "body":{
                    "book": bk
                }
            })),
            Err(err) => HttpResponse::InternalServerError().json(json!({
                "error":"Internal server error",
                "body":{
                    "error":err.to_string()
                }
            })),
        }
    }

    #[get("/book/{book_id}")]
    pub async fn find_book_by_id(book_id: web::Path<Uuid>) -> HttpResponse{
        let book = find_book(book_id.into_inner()).await;
        match book {
            Ok(bk) => match bk {
                Some(found_book) => HttpResponse::Ok().json(json!({
                    "message":"Book found",
                    "body":found_book
                })),
                None => HttpResponse::NotFound().json(json!({
                    "error":"Book not found",
                    "body":{}
                }))
            },
            Err(err) => HttpResponse::InternalServerError().json(json!({
                "error":err.to_string(),
                "body":{}
            }))
        }
    }

    #[get("/books")]
    pub async fn get_books() -> HttpResponse{
        let books = get_all_books().await;

        match books {
            Ok(bks) => HttpResponse::Ok().json(json!({
                "body":bks,
                "message":"Books retrieved"
            })),
            Err(err) => HttpResponse::InternalServerError().json(json!({
                "error":err.to_string()
            }))
        }
    }
}
