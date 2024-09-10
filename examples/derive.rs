use std::collections::HashMap;

use api_forge::ApiRequest;
use api_forge_macro::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[request(endpoint = "/posts", response_type = Posts)]
pub struct GetPosts;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Post {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Request)]
#[request(endpoint = "/posts", response_type = Post, method = POST, transmission = Json)]
pub struct CreatePost {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub title: String,
    pub body: String,

    #[request(header_name = "test")]
    #[serde(skip)]
    header: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Request)]
#[request(endpoint = "/posts/{id}", response_type = EmptyResponse, method = DELETE, path_parameters = ["id"])]
pub struct DeletePost {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Posts(Vec<Post>);

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EmptyResponse(HashMap<String, String>);

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

    // Initialize the request.
    let request = CreatePost {
        user_id: 1,
        title: "Test".to_string(),
        body: "Test".to_string(),
        header: Some("test-header".to_string()),
    };

    // Send the request and await the response.
    let result = request.send_and_parse(base_url, None, None).await;

    match result {
        Ok(post) => println!("Successfully created post: {:?}", post),
        Err(e) => eprintln!("Error occurred: {:?}", e),
    }

    // Initialize the request.
    let request = DeletePost { id: 100 };

    // Send the request and await the response.
    let result = request.send_and_parse(base_url, None, None).await;

    match result {
        Ok(post) => println!("Successfully deleted post: {:?}", post),
        Err(e) => eprintln!("Error occurred: {:?}", e),
    }
}
