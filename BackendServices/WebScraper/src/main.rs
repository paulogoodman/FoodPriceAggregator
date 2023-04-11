use postgres::{Client, NoTls};
use rust_webdriver::{Browser, Options, WebDriver, WebElement};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the PostgreSQL database
    let mut client = Client::connect("postgresql://user:password@localhost/dbname", NoTls)?;

    // Set options to run Chrome in headless mode
    let mut options = Options::new();
    options.set_headless(true);

    // Create a vector to store the threads
    let mut threads = Vec::new();

    // Spawn 4 threads to scrape URLs from the database concurrently
    for i in 0..4 {
        let thread_client = client.try_clone()?;
        let thread_options = options.clone();
        threads.push(thread::spawn(move || {
            // Launch a new Chrome browser instance with the specified options
            let mut driver = WebDriver::new_with_options(Browser::Chrome, &thread_options)?;

            // Loop through each URL in the database
            for row in thread_client.query("SELECT url FROM urls", &[])? {
                let url: String = row.get(0);
                println!("Thread {}: Scraping URL {}", i, url);

                // Navigate to the target web page
                driver.navigate_to(&url)?;

                // Get all the text content on the page
                let page_text = get_page_text(&mut driver)?;

                // Do something with the text content (e.g. save to a file or database)
                println!("{}", page_text);
            }

            Ok(())
        }));
    }

    // Wait for all threads to finish
    for thread in threads {
        thread.join().unwrap()?;
    }

    Ok(())
}

fn get_page_text(driver: &mut WebDriver) -> Result<String, Box<dyn std::error::Error>> {
    // Find the body element on the page
    let body = driver.find_element("body")?;

    // Get all the text content inside the body element
    let text = body.text()?;

    Ok(text)
}
