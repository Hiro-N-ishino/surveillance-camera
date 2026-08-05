use serde::Deserialize;
use std::fs;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub camera: CameraConfig,
    pub capture: CaptureConfig,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&contents)?;

        Ok(config)
    }
}

#[derive(Deserialize, Debug)]
pub struct CameraConfig {
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Debug)]
pub struct CaptureConfig {
    interval_sec: u64,
    pub max_consecutive_errors: u32,
}

impl CaptureConfig {
    pub fn interval_duration(&self) -> Duration {
        Duration::from_secs(self.interval_sec)
    }
}
