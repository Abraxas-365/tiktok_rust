use tiktok_rust::{error::TikTokApiError, user};
use tokio;

#[tokio::main]
async fn main() -> Result<(), TikTokApiError> {
    // Create a new Service instance for client and user services
    let user_service = user::Service::new();

    // Retrieve the access token (You need to get permissions)
    let token = "<you user token after oauth auth_example.rs>";

    let fields = vec![
        "open_id",
        "union_id",
        "avatar_url",
        "display_name",
        "bio_description",
        "is_verified",
        "follower_count",
        "following_count",
        "likes_count",
        "video_count",
    ];

    // Get user info
    match user_service.get_user_info(&token, fields).await {
        Ok(user_info) => {
            println!("Successfully retrieved user info:");
            println!("Open ID: {:?}", user_info.open_id);
            println!("Union ID: {:?}", user_info.union_id);
            println!("Avatar URL: {:?}", user_info.avatar_url);
            println!("Display Name: {:?}", user_info.display_name);
            println!("Bio: {:?}", user_info.bio_description);
            println!("Verified: {:?}", user_info.is_verified);
            println!("Follower Count: {:?}", user_info.follower_count);
            println!("Following Count: {:?}", user_info.following_count);
            println!("Likes Count: {:?}", user_info.likes_count);
            println!("Video Count: {:?}", user_info.video_count);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error getting user info: {:?}", e);
            Err(e)
        }
    }
}
