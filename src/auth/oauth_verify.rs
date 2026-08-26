use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
}

pub async fn verify_google_token(token: &str) -> Option<GoogleUserInfo> {
    let client = reqwest::Client::new();
    let url = format!("https://www.googleapis.com/oauth2/v1/userinfo?access_token={}", token);
    
    let res = client.get(&url).send().await.ok()?;
    res.json::<GoogleUserInfo>().await.ok()
}

pub async fn verify_github_token(token: &str) -> Option<GoogleUserInfo> {
    let client = reqwest::Client::new();
    let res = client.get("https://api.github.com/user")
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "Vella-Auth-Agent")
        .send().await.ok()?;
    
    let json: serde_json::Value = res.json().await.ok()?;
    Some(GoogleUserInfo {
        id: json["id"].as_i64().unwrap_or(0).to_string(),
        email: json["email"].as_str().unwrap_or("").to_string(),
        name: json["login"].as_str().unwrap_or("").to_string(),
    })
}
