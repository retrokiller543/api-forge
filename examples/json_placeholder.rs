use api_forge::ApiRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPosts;

impl ApiRequest<Vec<Post>> for GetPosts {
    const ENDPOINT: &'static str = "/posts";
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Post {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[tokio::main]
async fn main() {
    // Initialize the request.
    let request = GetPosts;

    // Define the base URL (e.g., JSONPlaceholder API for testing).
    let base_url = "https://jsonplaceholder.typicode.com";

    // Send the request and await the response.
    let result = request.send_and_parse(base_url, None, None).await;

    match result {
        Ok(post) => println!("Successfully fetched post: {:?}", post),
        Err(e) => eprintln!("Error occurred: {:?}", e),
    }
}
