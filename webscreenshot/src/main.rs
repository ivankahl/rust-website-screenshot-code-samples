use webscreenshotlib::{screenshot_tab, write_screenshot, OutputFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_data = screenshot_tab(
        "https://www.howtogeek.com/", 
        OutputFormat::PNG, 
        100, 
        true, 
        1920, 
        1080, 
        "")?;

    write_screenshot("screenshot.png", image_data)?;

    Ok(())
}
