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
