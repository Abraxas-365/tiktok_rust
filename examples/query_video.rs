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
    let token = "act.wFuFEXJGyEUHh6i3dSjwVQeAyb6YnFcDF8ONQ1R4U5p7qK2A6ldIUVG37eAv!5925.va";

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
