use tiktok_rust::videos;
use tiktok_rust::{error::TikTokApiError, videos::VideoField};
use tokio;

#[tokio::main]
async fn main() -> Result<(), TikTokApiError> {
    // Create a new Service instance for video service
    let video_service = videos::Service::new();

    // Retrieve the access token (You need to get permissions)
    let token = "<your user token after oauth auth_example.rs>";

    // Define pagination parameters
    let cursor = None; // Start from the beginning
    let max_count = Some(10); // Retrieve 10 videos

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

    // List videos
    match video_service
        .list_videos(&token, cursor, max_count, fields)
        .await
    {
        Ok(video_list_data) => {
            println!("Successfully retrieved video list:");
            for video in video_list_data.videos {
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
            println!("Cursor: {}", video_list_data.cursor);
            println!("Has more: {}", video_list_data.has_more);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error listing videos: {:?}", e);
            Err(e)
        }
    }
}
