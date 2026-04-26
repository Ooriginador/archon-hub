use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub prompt: String,
    pub model: Option<String>,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResponse {
    pub text: String,
    pub model_used: String,
    pub tokens_generated: usize,
}

pub struct ArchonClient {
    base_url: String,
    client: reqwest::Client,
}

impl ArchonClient {
    /// Create a new Archon client pointing to the local daemon.
    pub fn new(addr: &str) -> Self {
        Self {
            base_url: format!("http://{}", addr),
            client: reqwest::Client::new(),
        }
    }

    /// Create a client with default local address (127.0.0.1:8080).
    pub fn default() -> Self {
        Self::new("127.0.0.1:8080")
    }

    /// List available models on the daemon.
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/v1/models", self.base_url);
        let resp = self.client.get(url).send().await?.json::<Vec<String>>().await?;
        Ok(resp)
    }

    /// Generate text using the intelligent routing system.
    pub async fn generate(&self, prompt: &str) -> Result<GenerateResponse> {
        let req = GenerateRequest {
            prompt: prompt.to_string(),
            model: None,
            max_tokens: None,
            temperature: None,
        };
        self.generate_with_config(req).await
    }

    /// Generate text with full configuration.
    pub async fn generate_with_config(&self, req: GenerateRequest) -> Result<GenerateResponse> {
        let url = format!("{}/v1/generate", self.base_url);
        let resp = self.client.post(url)
            .json(&req)
            .send()
            .await?
            .json::<GenerateResponse>()
            .await?;
        Ok(resp)
    }
}
