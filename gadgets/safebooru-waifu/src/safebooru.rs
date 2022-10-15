use egui_extras::RetainedImage;
use serde::{Deserialize, Serialize};
use utils::RetainedImageError;

const QUERY_URL: &str = "https://safebooru.donmai.us/posts/random.json?tags=1girl";

#[derive(Serialize, Deserialize)]
struct SafebooruResponse {
    pub file_url: String,
}

pub async fn query_random_image() -> anyhow::Result<RetainedImage> {
    let response = reqwest::get(QUERY_URL).await?.text().await?;
    let response: SafebooruResponse = serde_json::from_str(&response)?;
    let image_response = reqwest::get(response.file_url).await?;

    let image = RetainedImage::from_image_bytes(
        image_response.url().to_string(),
        &image_response.bytes().await?,
    )
    .map_err(RetainedImageError)?;

    Ok(image)
}
