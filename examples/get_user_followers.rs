use tiktok_rust::{
    client,
    error::TikTokApiError,
    research::{self, ResearchUserFollowersRequest},
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), TikTokApiError> {
    // Create a new Service instance
    let research_service = research::Service::new();

    let client_service = client::Service::new();

    // Retrieve the access token You need to get permissions
    let token = match client_service.get_access_token().await {
        Ok(token_response) => {
            println!("Access Token: {}", token_response.access_token);
            println!("Expires In: {}", token_response.expires_in);
            println!("Token Type: {}", token_response.token_type);
            token_response.access_token
        }
        Err(e) => {
            eprintln!("Error getting access token: {}", e);
            "".to_string()
        }
    };

    // Create a UserFollowersRequest
    let request = ResearchUserFollowersRequest {
        username: "luisf_m99".to_string(),
        max_count: Some(20), // Number of followers to retrieve (adjust as needed)
        cursor: Some(0),     // Starting point for pagination (0 for the first request)
    };

    // Query user followers
    match research_service.query_user_followers(&token, request).await {
        Ok(follower_data) => {
            println!("Successfully retrieved follower data:");
            println!("Cursor: {}", follower_data.cursor);
            println!("Has more: {}", follower_data.has_more);

            for follower in follower_data.user_followers {
                println!("Username: {}", follower.username);
                println!("Display name: {}", follower.display_name);
                println!("------------------------");
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error querying user followers: {:?}", e);
            Err(e)
        }
    }
}
