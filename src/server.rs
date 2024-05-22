use std::array;

#[tokio::main]
pub async fn get_user_list(
    hostname: String,
    authcode: String,
    https: bool,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let protocol = if https { "https://" } else { "http://" };

    let url = format!("{protocol}{hostname}:{port}/users/get?ac={authcode}");

    println!("{}", url);

    let response = reqwest::get(url).await?.text().await?;

    println!("{response:#?}");

    let json = serde_json::from_str(response.as_str());

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    valid: bool,
    data: ,
}
