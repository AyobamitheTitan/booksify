pub mod user{
    use actix_web::{post, web, HttpResponse};
    use schema::user::user::{DisplayUser, Login, NewUser};
    use serde_json::json;
    use service::{helper::{password::password::verify_password, token::token::issue_token}, mutation::user::user::create_user, query::user::user::get_user};
    use utoipa;

    #[utoipa::path(
        post,
        path = "/register",
        responses(
            (status = 200, description = "New user Added")
        ),
        request_body = NewUser,
    )]
    #[post("/register")]
    pub async fn register(payload: web::Json<NewUser>) -> HttpResponse {
        let new_user = create_user(payload.0).await;

        HttpResponse::Ok().json(json!({
            "body": {
                "user":DisplayUser{
                id: new_user.id.unwrap(),
                first_name: new_user.first_name.unwrap(),
                last_name: new_user.last_name.unwrap(),
                email: new_user.email.unwrap(),
                dob: new_user.dob.unwrap()
            }},
            "message":"New user added"
        }))
    }

    #[post("/login")]
    pub async fn login(payload: web::Json<Login>) -> HttpResponse{
        let user = get_user(&payload.0.email.as_str()).await;
        
    
        match user {
            Some(usr) => {
                match verify_password(&payload.password, &usr.password) {
                    Ok(_) => match issue_token(&payload.email) {
                        Ok(jwt_token) => HttpResponse::Ok().json(json!({
                            "message":"Logged in",
                            "body":{
                                "token":jwt_token
                            }
                        })),
                        Err(err) => HttpResponse::InternalServerError().json(json!({
                            "message":err.to_string(),
                            "body":{}
                        }))
                    },
                    Err(err) => HttpResponse::BadRequest().json(json!({
                        "message":err.to_string(),
                        "body":{
                        }
                    }))
                }
            },
            None => HttpResponse::NotFound().json(json!({
                "error":"User not found",
                "body":{}
            }))
        }
    }
}