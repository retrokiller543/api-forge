use serde::{Serialize, Deserialize};
use api_forge::ApiRequest;
use api_forge_macro::Request;

const BASE_URL: &str = "https://jsonplaceholder.typicode.com";

#[derive(serde::Serialize, Request, Debug)]
#[request(endpoint = "/posts", transmission = Json, response_type = "Vec<Posts>")]
struct GetPosts;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Posts {
	#[serde(rename = "userId")]
	pub user_id: i32,

	#[serde(rename = "id")]
	pub id: i32,

	#[serde(rename = "title")]
	pub title: String,

	#[serde(rename = "body")]
	pub body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	
	let request = GetPosts;

	let res = request.send_request(BASE_URL, None, None).await;

	match res {
		Ok(posts) => println!("{:?}", posts.json::<Vec<Posts>>().await?),
		Err(e) => panic!("{:?}", e),
	}
	
	Ok(())
}