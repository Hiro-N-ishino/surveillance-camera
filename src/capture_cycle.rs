use crate::camera::Camera;
use crate::config::UploadConfig;
use crate::fs;
use crate::upload;

pub fn run(
    camera: &Camera,
    upload_config: &UploadConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = fs::create_file_name();

    camera.capture(&file_name)?;

    println!("Saved as \"{}\"", &file_name);

    upload::upload(&file_name, &upload_config.worker_url)?;

    Ok(())
}
