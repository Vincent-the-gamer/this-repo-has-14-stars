use serde_json::Value;

pub async fn request_github() -> Value {
    let url: &str = "https://api.github.com/search/repositories?q=Vincent-the-gamer/this-repo-has";

    let client = reqwest::Client::new();

    let resp: Value = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await.unwrap()
        .json()
        .await.unwrap();
    resp
}