use std::error::Error;
use std::fs;

pub fn upload(file_name: &str, worker_url: &str) -> Result<(), Box<dyn Error>> {
    let image = fs::read(file_name)?;

    println!("Image loaded: {} bytes", image.len());

    let client = reqwest::blocking::Client::new();
    let url = format!("{}?filename={}", worker_url, file_name);
    let response = client.post(&url).body(image).send()?;

    if response.status().is_success() {
        println!("Upload succeeded.");
        Ok(())
    } else {
        Err(format!("Upload failed: {}", response.status()).into())
    }
}
