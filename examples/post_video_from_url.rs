use tiktok_rust::direct_post::{self, PostInfoBuilder, PrivacyLevel};

#[tokio::main]
async fn main() {
    // Retrieve the access token
    let token = "Your user token";
    // Create an instance of the Service
    let service = direct_post::Service::new();

    // Build PostInfo
    let post_info = PostInfoBuilder::default()
        .title("Check out this amazing video!")
        .privacy_level(PrivacyLevel::PublicToEveryone)
        .disable_duet(false)
        .disable_comment(false)
        .disable_stitch(false)
        .video_cover_timestamp_ms(1000u64)
        .build()
        .unwrap();

    // Use a random video URL from the internet
    let video_url =
        "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";

    // Upload the video from URL
    match service
        .upload_video_from_url(&token, post_info, video_url)
        .await
    {
        Ok(status_data) => println!("Post Status: {:?}", status_data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
