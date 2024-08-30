use tiktok_rust::client;

#[tokio::main]
async fn main() {
    // Set the environment variables for client_key and client_secret
    // For example, in a real application, you would set these in your environment or configuration
    // env::set_var("TIKTOK_CLIENT_KEY", "your_client_key");
    // env::set_var("TIKTOK_CLIENT_SECRET", "your_client_secret");

    // Create an instance of the Service
    let service = client::Service::new();

    // Retrieve the access token
    match service.get_access_token().await {
        Ok(token_response) => {
            println!("Access Token: {}", token_response.access_token);
            println!("Expires In: {}", token_response.expires_in);
            println!("Token Type: {}", token_response.token_type);
        }
        Err(e) => {
            eprintln!("Error getting access token: {}", e);
        }
    }
}
