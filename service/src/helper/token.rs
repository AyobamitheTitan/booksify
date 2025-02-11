pub mod token{
    use std::time::Duration;

    use jwts::{jws::{self, alg::HS256, Header}, Claims};

    pub fn issue_token(sub: &str) -> Result<std::string::String, jwts::Error>{
        dotenv::dotenv().unwrap();

        let claims = Claims{
            sub: Some(sub.to_string()),
            ..Default::default()
        };
        
        let claims = claims
        .issued_now()
        .expired_in(Duration::from_secs(7200)); // 2 hours

        match std::env::var("JWT_KEY"){
            Ok(key) => jws::encode::<HS256>(
                Header::default(),
                &claims,
                key.as_bytes()
            ),
            Err(_) => Err(jwts::Error::InvalidKey("")),
        }
    }
}