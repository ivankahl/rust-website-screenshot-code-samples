use std::fs;
use std::thread::sleep;
use std::time::Duration;
use headless_chrome::{LaunchOptionsBuilder, Browser, protocol::page::ScreenshotFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the launch options for Chrome before
    // starting the browser.
    let options = LaunchOptionsBuilder::default()
        // Make the window bigger
        .window_size(Some((1920, 1080)))
        .build()?;

    // Open a new instance of Chrome with the specified
    // options
    let browser = Browser::new(options)?;

    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://www.howtogeek.com/")?;

    tab.wait_until_navigated()?;

    // Sleep for some more seconds to make sure everything
    // has loaded
    sleep(Duration::from_secs(5));

    let png_data = tab.capture_screenshot(
        ScreenshotFormat::PNG, 
        None, 
        true)?;

    fs::write("screenshot.png", png_data)?;

    Ok(())
}