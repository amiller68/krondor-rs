use std::fs;
use std::path::{Path, PathBuf};
use reqwest;

pub struct NodeClient {
    auth: Auth,
    endpoint: String,
}

pub struct GatewayClient(String);

struct Auth {
    username: Option<String>,
    password: Option<String>,
}

// impl NodeClient {
//     pub fn new(endpoint: String, api_key: Option<String>, api_secret: Option<String>) -> Self {
//         Self {
//             auth: Auth {
//                 username: api_key,
//                 password: api_secret,
//             },
//             endpoint,
//         }
//     }

//     #[cfg(not(target_arch = "wasm32"))]
//     pub fn pin_file(&self, file_path: &Path) -> Result<String, reqwest::Error> {
//         let form = reqwest::blocking::multipart::Form::new()
//             .add_file("file", file_path).unwrap()
//             .prepare().unwrap();

//         let url = format!("{}/api/v0/add?pin=1&cid-version=1", self.endpoint);

//         let client = reqwest::blocking::Client::new();
//         let resp: serde_json::Value = client.post(&url)
//             .basic_auth(self.auth.username.as_deref().unwrap_or_default(), self.auth.password.as_ref())
//             .multipart(form)
//             .send()?
//             .json()?;

//         Ok(resp["Hash"].as_str().unwrap().to_string())
//     }

//     pub fn pull_file(&self, cid: &str, file_path: &Path) -> Result<(), reqwest::Error> {
//         let url = format!("{}/api/v0/block/get?arg={}", self.endpoint, cid);

//         let client = reqwest::blocking::Client::new();
//         let mut resp = client.post(&url)
//             .basic_auth(self.auth.username.as_deref().unwrap_or_default(), self.auth.password.as_ref())
//             .send()?;

//         let mut out_file = fs::File::create(file_path).expect("Unable to create file");
//         resp.copy_to(&mut out_file).expect("Failed to copy content");
//         Ok(())
//     }

//     #[cfg(not(target_arch = "wasm32"))]
//     pub fn pin_directory(&self, dir_path: &Path) -> Result<Vec<serde_json::Value>, reqwest::Error> {
//         let mut form = Multipart::new();

//         for entry in fs::read_dir(dir_path).expect("Unable to read dir") {
//             let entry = entry.expect("Dir entry failed");
//             if entry.path().is_file() {
//                 form.add_file(entry.file_name().to_str().unwrap(), &entry.path()).unwrap();
//             }
//         }

//         let url = format!("{}/api/v0/add?pin=1&recursive=1&cid-version=1&wrap-with-directory=1", self.endpoint);

//         let client = reqwest::blocking::Client::new();
//         let resp_text = client.post(&url)
//             .basic_auth(self.auth.username.as_deref().unwrap_or_default(), self.auth.password.as_ref())
//             .multipart(form.prepare().unwrap())
//             .send()?
//             .text()?;

//         let lines: Vec<&str> = resp_text.split('\n').collect();
//         let mut results = vec![];

//         for line in lines {
//             if !line.is_empty() {
//                 let parsed: serde_json::Value = serde_json::from_str(line)?;
//                 results.push(parsed);
//             }
//         }
//         Ok(results)
//     }
// }

impl GatewayClient {
    pub async fn get(&self, cid: &str) -> Result<Vec<u8>, reqwest::Error> {
        let url = format!("{}/ipfs/{}", self.0, cid);
        let client = reqwest::Client::new();
        let resp = client.get(&url).send().await.expect("Failed to send request");
        let bytes = resp.bytes().await.expect("Failed to get bytes");
        Ok(bytes.to_vec())
    }
}