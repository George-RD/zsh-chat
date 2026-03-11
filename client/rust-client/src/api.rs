use reqwest::Client;
use crate::models::Post;

pub struct ApiClient {
    client: Client,
    api_url: String,
}

impl ApiClient {
    pub fn new(api_url: String) -> Self {
        Self {
            client: Client::new(),
            api_url,
        }
    }

    pub async fn fetch_posts(&self) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
        let posts: Vec<Post> = self.client
            .get(format!("{}/posts", self.api_url))
            .send()
            .await?
            .json()
            .await?;
        Ok(posts)
    }

    pub async fn create_post(&self, post: &Post) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.client
            .post(format!("{}/posts", self.api_url))
            .json(post)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to post: {}", res.status()).into())
        }
    }
}
