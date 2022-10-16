use egui_extras::RetainedImage;
use serde::{Deserialize, Serialize};
use utils::image::RetainedImageError;

const QUERY_URL: &str = "https://safebooru.donmai.us/posts/random.json?tags=1girl";
const POST_URL: &str = "https://safebooru.donmai.us/posts/";

#[derive(Serialize, Deserialize)]
struct SafebooruResponse {
    id: u64,
    file_url: String,
}

pub struct ImageWithMetadata {
    pub inner: RetainedImage,
    pub file_url: String,
    pub post_url: String,
}

pub async fn query_random_image() -> anyhow::Result<ImageWithMetadata> {
    let response = reqwest::get(QUERY_URL).await?.text().await?;
    let response: SafebooruResponse = serde_json::from_str(&response)?;
    let image_response = reqwest::get(&response.file_url).await?;

    let image = RetainedImage::from_image_bytes(
        image_response.url().to_string(),
        &image_response.bytes().await?,
    )
    .map_err(RetainedImageError)?;

    Ok(ImageWithMetadata {
        inner: image,
        file_url: response.file_url,
        post_url: format!("{}{}", POST_URL, response.id),
    })
}
