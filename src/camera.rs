use chrono::Local;
use std::process::Command;

/*
#[derive(Debug)]
pub enum CameraError {
    CommandError(String),
    CaptureFailed,
    ConvertError(String),
}
*/

pub struct Camera {
    width: u32,
    height: u32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        Ok(Self { width, height })
    }

    pub fn create_file_name() -> String {
        let now = Local::now();
        format!("{}.jpg", now.format("%Y%m%d_%H%M%S"))
    }

    pub fn print_info(&self) {
        println!("Resolution: {} x {}", self.width, self.height);
    }

    pub fn capture(&self, file_name: &str) -> Result<(), String> {
        let output = Command::new("rpicam-still")
            .arg("-o")
            .arg(file_name)
            .arg("--nopreview")
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            //Err(String::from("Capture failed."))
            Err(String::from_utf8(output.stderr).map_err(|e| e.to_string())?)
        }
    }
}
