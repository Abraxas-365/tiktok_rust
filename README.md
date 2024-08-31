# TikTok Rust API Client

A Rust library for interacting with the TikTok API, enabling direct posting of content and querying creator information.

## Features

-   Query creator information
-   Post videos to TikTok
-   Upload video files
-   Check the status of video posts
-   Simplified functions for uploading videos from files or URLs
-   Query user followers
-   Query user following
-   Query reposted videos

## Usage

**Auth User**
```rust
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
```

**Query User Videos**
```rust
use tiktok_rust::{
    error::TikTokApiError,
    videos::{self, VideoField},
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), TikTokApiError> {
    // Create a new Service instance for video service
    let video_service = videos::Service::new();

    // Retrieve the access token (You need to get permissions)
    let token = "<your user token after oauth auth_example.rs>";

    // Define video IDs to query
    let video_ids = vec![
        "1234123412345678567".to_string(),
        "1010102020203030303".to_string(),
    ];

    // Define fields to retrieve
    let fields = vec![
        VideoField::Id,
        VideoField::Title,
        VideoField::CreateTime,
        VideoField::CoverImageUrl,
        VideoField::ShareUrl,
        VideoField::ViewCount,
        VideoField::LikeCount,
        VideoField::CommentCount,
    ];

    // Query videos
    match video_service.query_videos(&token, video_ids, fields).await {
        Ok(videos) => {
            println!("Successfully retrieved video info:");
            for video in videos {
                println!("Video ID: {}", video.id);
                println!("Title: {:?}", video.title);
                println!("Create Time: {:?}", video.create_time);
                println!("Cover Image URL: {:?}", video.cover_image_url);
                println!("Share URL: {:?}", video.share_url);
                println!("View Count: {:?}", video.view_count);
                println!("Like Count: {:?}", video.like_count);
                println!("Comment Count: {:?}", video.comment_count);
                println!("------------------------");
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error querying videos: {:?}", e);
            Err(e)
        }
    }
}
```

**Query Creator Information**

```rust
use tiktok::creator::Service as CreatorService;

#[tokio::main]
async fn main() {
    let service = CreatorService::new();

    let token = "your_api_token";

    match service.get_creator_info(token).await {
        Ok(creator_data) => println!("Creator Info: {:?}", creator_data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**Post a Video**

```rust
use tiktok::direct_post::{PostInfoBuilder, PrivacyLevel, SourceInfoBuilder, Service as DirectPostService, Source};

#[tokio::main]
async fn main() {
    let service = DirectPostService::new();

    let token = "your_api_token";

    // Build PostInfo
    let post_info = PostInfoBuilder::default()
        .title("this will be a funny #cat video on your @tiktok #fyp")
        .privacy_level(PrivacyLevel::MutualFollowFriends)
        .disable_duet(false)
        .disable_comment(true)
        .disable_stitch(false)
        .video_cover_timestamp_ms(1000)
        .build()
        .unwrap();

    // Build SourceInfo for FILE_UPLOAD
    let source_info = SourceInfoBuilder::default()
        .source(Source::FileUpload)
        .video_size(Some(50000123))
        .chunk_size(Some(10000000))
        .total_chunk_count(Some(5))
        .build()
        .unwrap();

    // Create VideoInitRequest
    let video_init_request = VideoInitRequestBuilder::default()
        .post_info(post_info)
        .source_info(source_info)
        .build()
        .unwrap();

    // Post the video
    match service.post_video(token, video_init_request).await {
        Ok(response_data) => println!("Video Init Response: {:?}", response_data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**Upload Video File**

```rust
use tiktok::direct_post::Service as DirectPostService;

#[tokio::main]
async fn main() {
    let service = DirectPostService::new();

    let token = "your_api_token";

    // Upload the video file
    match service.upload_video(token, "upload_url", "/path/to/file/example.mp4").await {
        Ok(_) => println!("Video uploaded successfully"),
        Err(e) => eprintln!("Error uploading video: {}", e),
    }
}
```

**Check Post Status**

```rust
use tiktok::direct_post::Service as DirectPostService;

#[tokio::main]
async fn main() {
    let service = DirectPostService::new();

    let token = "your_api_token";

    // Check the post status
    match service.get_post_status(token, "publish_id").await {
        Ok(status_data) => println!("Post Status: {:?}", status_data),
        Err(e) => eprintln!("Error getting post status: {}", e),
    }
}
```

**Simplified Function to Upload Video from File**

```rust
use tiktok::direct_post::{PostInfoBuilder, PrivacyLevel, Service as DirectPostService};

#[tokio::main]
async fn main() {
    let service = DirectPostService::new();

    let token = "your_api_token";

    // Build PostInfo
    let post_info = PostInfoBuilder::default()
        .title("this will be a funny #cat video on your @tiktok #fyp")
        .privacy_level(PrivacyLevel::MutualFollowFriends)
        .disable_duet(false)
        .disable_comment(true)
        .disable_stitch(false)
        .video_cover_timestamp_ms(1000)
        .build()
        .unwrap();

    // Upload the video from file
    match service.upload_video_from_file(
        token,
        post_info,
        "/path/to/file/example.mp4",
        50000123,
        10000000,
        5,
    ).await {
        Ok(status_data) => println!("Post Status: {:?}", status_data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**Simplified Function to Upload Video from URL**

```rust
use tiktok::direct_post::{PostInfoBuilder, PrivacyLevel, Service as DirectPostService};

#[tokio::main]
async fn main() {
    let service = DirectPostService::new();

    let token = "your_api_token";

    // Build PostInfo
    let post_info = PostInfoBuilder::default()
        .title("this will be a funny #cat video on your @tiktok #fyp")
        .privacy_level(PrivacyLevel::MutualFollowFriends)
        .disable_duet(false)
        .disable_comment(true)
        .disable_stitch(false)
        .video_cover_timestamp_ms(1000)
        .build()
        .unwrap();

    // Upload the video from URL
    match service.upload_video_from_url(
        token,
        post_info,
        "https://example.verified.domain.com/example_video.mp4",
    ).await {
        Ok(status_data) => println!("Post Status: {:?}", status_data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**Query User Followers**

```rust
use tiktok::user::Service as UserService;

#[tokio::main]
async fn main() {
    let service = UserService::new();

    let token = "your_api_token";
    let user_followers_request = UserFollowersRequest {
        username: "test_user".to_string(),
        max_count: Some(3),
        cursor: Some(12347764),
    };

    match service.query_user_followers(token, user_followers_request).await {
        Ok(user_followers_data) => println!("User Followers Data: {:?}", user_followers_data),
        Err(e) => eprintln!("Error querying user followers: {}", e),
    }
}
```

**Query User Following**

```rust
use tiktok::user::Service as UserService;

#[tokio::main]
async fn main() {
    let service = UserService::new();

    let token = "your_api_token";
    let user_following_request = UserFollowingRequest {
        username: "test_user".to_string(),
        max_count: Some(3),
        cursor: Some(1685544251),
    };

    match service.query_user_following(token, user_following_request).await {
        Ok(user_following_data) => println!("User Following Data: {:?}", user_following_data),
        Err(e) => eprintln!("Error querying user following: {}", e),
    }
}
```

**Query Reposted Videos**

```rust
use tiktok::user::Service as UserService;

#[tokio::main]
async fn main() {
    let service = UserService::new();

    let token = "your_api_token";
    let fields = vec![
        VideoField::Id,
        VideoField::CreateTime,
        VideoField::Username,
        VideoField::RegionCode,
        VideoField::VideoDescription,
        VideoField::MusicId,
        VideoField::LikeCount,
        VideoField::CommentCount,
        VideoField::ShareCount,
        VideoField::ViewCount,
        VideoField::HashtagNames,
        VideoField::VideoDuration,
        VideoField::IsStemVerified,
        VideoField::FavouritesCount,
    ];
    let reposted_videos_request = RepostedVideosRequest {
        username: "test_username".to_string(),
        max_count: Some(6),
        cursor: None,
    };

    match service.query_reposted_videos(token, &fields, reposted_videos_request).await {
        Ok(reposted_videos_data) => println!("Reposted Videos Data: {:?}", reposted_videos_data),
        Err(e) => eprintln!("Error querying reposted videos: {}", e),
    }
}
```
