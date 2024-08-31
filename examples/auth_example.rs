use std::sync::Arc;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use tiktok_rust::auth::{AuthCallback, AuthService, OAuthConfig, TikTokScope};

const CLIENT_KEY: &str = "your_key";
const CLIENT_SECRET: &str = "your_secret"; // Make sure to set this
const REDIRECT_URI: &str =
    "https://df12-2001-1388-19-4828-4a3-c2fb-8272-1935.ngrok-free.app/auth/callback";

struct AppState {
    auth_service: Arc<AuthService>,
}

#[get("/oauth")]
async fn oauth(data: web::Data<AppState>) -> impl Responder {
    let auth_url = data.auth_service.get_authorization_url();

    println!("Full Authorization URL: {}", auth_url);

    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

#[get("/auth/callback")]
async fn callback(query: web::Query<AuthCallback>, data: web::Data<AppState>) -> impl Responder {
    println!("Received callback with query params: {:?}", query);

    match data
        .auth_service
        .validate_callback(&query.clone().into_inner())
    {
        Ok(_message) => {
            // Exchange the code for an access token
            match &query.code {
                Some(code) => {
                    match data
                        .auth_service
                        .fetch_access_token(code, REDIRECT_URI, None)
                        .await
                    {
                        Ok(token_response) => {
                            // Here you would typically store the token securely
                            HttpResponse::Ok().body(format!(
                                "Access token obtained: {}",
                                token_response.access_token
                            ))
                        }
                        Err(e) => HttpResponse::InternalServerError()
                            .body(format!("Failed to obtain access token: {}", e)),
                    }
                }
                None => HttpResponse::BadRequest().body("No authorization code provided"),
            }
        }
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080");
    println!("Client Key: {}", CLIENT_KEY);
    println!("Redirect URI: {}", REDIRECT_URI);

    let oauth_config = OAuthConfig::new(
        CLIENT_KEY,
        CLIENT_SECRET,
        REDIRECT_URI,
        &[
            TikTokScope::UserInfoProfile,
            TikTokScope::UserInfoStats,
            TikTokScope::VideoList,
            TikTokScope::VideoUpload,  //You need to add video to you app
            TikTokScope::VideoPublish, //You need to add video to you app
        ],
    );

    let auth_service = Arc::new(AuthService::new(oauth_config));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                auth_service: Arc::clone(&auth_service),
            }))
            .service(oauth)
            .service(callback)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
