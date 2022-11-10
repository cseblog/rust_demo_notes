use std::error::Error;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
struct Post {
    Id: u64,
    Title: String,
    Body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:9990/search")
        .body("law")
        .send()
        .await?;

    let js = res.json::<Vec<Post>>().await?;
    println!("{}", serde_json::to_string(&js)?);

    let res2 = client
        .post("http://127.0.0.1:9990/search/101203")
        .send()
        .await?;
    let rj = res2.json::<Post>().await?;
    println!("{}", serde_json::to_string(&rj)?);
    Ok(())
}
