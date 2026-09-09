use crate::observation::Observation;
use std::error::Error;
use std::fs;

pub fn upload(observation: &Observation, worker_url: &str) -> Result<(), Box<dyn Error>> {
    let image = fs::read(&observation.file_name)?;

    println!("Image loaded: {} bytes", image.len());

    let username = std::env::var("CAMERA_USERNAME")?;
    let password = std::env::var("CAMERA_PASSWORD")?;

    let client = reqwest::blocking::Client::new();
    let url = format!("{}?filename={}", worker_url, observation.file_name);
    let response = client
        .post(&url)
        .basic_auth(username, Some(password))
        .header("X-Temperature", observation.temperature.to_string())
        .header("X-Humidity", observation.humidity.to_string())
        .body(image)
        .send()?;

    if response.status().is_success() {
        println!("Upload succeeded.");
        Ok(())
    } else {
        Err(format!("Upload failed: {}", response.status()).into())
    }
}
