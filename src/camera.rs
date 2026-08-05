use crate::config::CameraConfig;
use std::fmt;
use std::process::Command;

#[derive(Debug)]
pub enum CameraError {
    InvalidResolution,
    CommandError(String),
    CaptureFailed,
}

impl fmt::Display for CameraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CameraError::InvalidResolution => write!(f, "Resolution must be greater than zero."),
            CameraError::CommandError(msg) => write!(f, "{}", msg),
            CameraError::CaptureFailed => write!(f, "Capture Failed"),
        }
    }
}

impl std::error::Error for CameraError {}

pub struct Camera {
    width: u32,
    height: u32,
}

impl Camera {
    pub fn new(config: &CameraConfig) -> Result<Self, CameraError> {
        if config.width == 0 || config.height == 0 {
            return Err(CameraError::InvalidResolution);
        }
        Ok(Self {
            width: config.width,
            height: config.height,
        })
    }

    pub fn print_info(&self) {
        println!("Resolution: {} x {}", self.width, self.height);
    }

    pub fn capture(&self, file_name: &str) -> Result<(), CameraError> {
        let output = Command::new("rpicam-still")
            .arg("-o")
            .arg(file_name)
            .arg("--nopreview")
            .output()
            .map_err(|e| CameraError::CommandError(e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(CameraError::CaptureFailed)
        }
    }
}
