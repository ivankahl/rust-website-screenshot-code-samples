use futures_util::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};
use reqwest::Client;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new reqwest client which we will use for our REST
    // calls to the Urlbox API
    let client = Client::new();

    // Retrieve our API Key from our environment variables
    let api_key = env!("URLBOX_API_KEY");

    // Format our REST API URL with our API Key
    let api_url = format!("https://api.urlbox.io/v1/{api_key}/png");

    // Use the reqwest client to call the REST API URL above
    // and return the response as a byte stream which we can
    // save to a file.
    let mut stream = client.get(api_url)
        .query(&[
            // The URL we want to screenshot
            ("url", "https://www.howtogeek.com/"),
            // Specify the screen width to use
            ("width", "1920"),
            // Screenshot the entire page
            ("full_page", "true"),
            // Click accept on any popups
            ("click_accept", "true"),
            // Block any ads that might appear on the page
            ("block_ads", "true"),
            // Hide any cookie banners that might appear
            ("hide_cookie_banners", "true"),
            // Screenshot the webpage in retina quality
            ("retina", "true"),
            // Hide a notification dialog that appears as well as
            // the empty space where ads appeared
            ("hide_selector", "#notificationAllowPrompt, .adslot")
        ])
        .send()
        .await?
        .bytes_stream();

    // Create a new file that we can write the response bytes to
    let mut file = File::create("screenshot.png").await?;

    // Write the bytes for the screenshot image to a file
    while let Some(item) = stream.next().await {
        file.write_all_buf(&mut item?).await?;
    }

    Ok(())
}