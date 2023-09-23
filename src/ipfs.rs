#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

use anyhow::Result;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::Body;

#[cfg(not(target_arch = "wasm32"))]
use futures_util::StreamExt;
#[cfg(not(target_arch = "wasm32"))]
use tokio::fs;

#[cfg(not(target_arch = "wasm32"))]
pub struct NodeClient {
    auth: Auth,
    endpoint: String,
}

#[derive(Clone, Debug)]
pub struct GatewayClient(pub String);

#[cfg(not(target_arch = "wasm32"))]
struct Auth {
    username: Option<String>,
    password: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NodeClient {
    pub fn new(endpoint: String, api_key: Option<String>, api_secret: Option<String>) -> Self {
        Self {
            auth: Auth {
                username: api_key,
                password: api_secret,
            },
            endpoint,
        }
    }

    pub async fn pin_file(&self, file_path: &Path) -> Result<String> {
        let file: fs::File = fs::File::open(file_path).await?;
        let body = Body::from(file);
        let filename = Path::new(file_path)
            .file_name()
            .unwrap()
            .to_str()
            .map(|s| s.to_string())
            .unwrap();
        println!("Filename: {}", filename);
        // let file_part = reqwest::multipart::Part::stream(body)
        //     .file_name(filename)
        let file_part = reqwest::multipart::Part::stream(body)
            .file_name(filename.clone())
            .mime_str("application/octet-stream")?; // or any other MIME type as appropriate

        let form = reqwest::multipart::Form::new().part(format!("{}-part", filename), file_part);

        let url = format!("{}/api/v0/add?pin=1&cid-version=1", self.endpoint);

        let client = reqwest::Client::new();
        let builder = client.post(&url).multipart(form).basic_auth(
            self.auth.username.as_deref().unwrap_or_default(),
            self.auth.password.as_ref(),
        );
        let resp = builder.send().await?;
        if resp.status().is_success() {
            let text = resp.text().await?;
            println!("Text: {:?}", text);
            let value: serde_json::Value = serde_json::from_str(&text)?;
            Ok(value["Hash"].as_str().unwrap().to_string())
        } else {
            Err(anyhow::anyhow!("Failed to pin file"))
        }
    }

    pub async fn pull_file(&self, cid: &str, file_path: &Path) -> Result<()> {
        let url = format!("{}/api/v0/block/get?arg={}", self.endpoint, cid);

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .basic_auth(
                self.auth.username.as_deref().unwrap_or_default(),
                self.auth.password.as_ref(),
            )
            .send()
            .await?;

        let mut out_file = fs::File::create(file_path).await?;
        let mut stream = resp.bytes_stream();
        while let Some(chunk) = stream.next().await {
            tokio::io::copy(&mut chunk.unwrap().as_ref(), &mut out_file).await?;
        }
        Ok(())
    }

    pub async fn pin_directory(&self, dir_path: &Path) -> Result<String> {
        let mut form = reqwest::multipart::Form::new();
        let mut entries = fs::read_dir(dir_path).await.expect("Unable to read dir");
        while let Some(entry) = entries.next_entry().await.expect("Dir entry failed") {
            if entry.path().is_file() {
                let file = fs::File::open(entry.path()).await?;
                let body = Body::from(file);
                let filename = Path::new(&entry.path())
                    .file_name()
                    .unwrap()
                    .to_str()
                    .map(|s| s.to_string())
                    .unwrap();
                let file_part = reqwest::multipart::Part::stream(body)
                    .file_name(filename.clone())
                    .mime_str("application/octet-stream")?;
                form = form.part(format!("{}-part", filename), file_part)
            }
        }

        let url = format!(
            "{}/api/v0/add?pin=1&recursive=1&cid-version=1&wrap-with-directory=1",
            self.endpoint
        );

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .basic_auth(
                self.auth.username.as_deref().unwrap_or_default(),
                self.auth.password.as_ref(),
            )
            .multipart(form)
            .send()
            .await?;

        if resp.status().is_success() {
            let mut results = vec![];
            let text = resp.text().await?;
            let lines: Vec<&str> = text.split('\n').collect();
            for line in lines {
                if !line.is_empty() {
                    let parsed: serde_json::Value = serde_json::from_str(line)?;
                    results.push(parsed.clone());
                }
            }
            // Find the directory entry -- it has Name == ""
            let dir_entry = results
                .iter()
                .find(|entry| entry["Name"].as_str().unwrap() == "")
                .unwrap();
            Ok(dir_entry["Hash"].as_str().unwrap().to_string())
        } else {
            Err(anyhow::anyhow!("Failed to pin file"))
        }
    }
}

impl GatewayClient {
    pub async fn get(&self, cid: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/ipfs/{}", self.0, cid);
        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .send()
            .await
            .expect("Failed to send request");
        let text = resp.text().await.expect("Failed to get text");
        Ok(text)
    }
}
