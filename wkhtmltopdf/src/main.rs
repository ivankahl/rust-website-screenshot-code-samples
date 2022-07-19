use wkhtmltopdf::{ImageApplication, ImageFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new image application that we will use 
    let image_app = ImageApplication::new()?;

    // Use the image application above to take a PNG screenshot
    // of the specified URL
    let mut image_out = image_app.builder()
       .format(ImageFormat::Png)
       .screen_width(1920)
       .build_from_url(&"https://www.howtogeek.com/".parse().unwrap())?;

    // Save the new screenshot to a file
    image_out.save("screenshot.png")?;

    Ok(())
}