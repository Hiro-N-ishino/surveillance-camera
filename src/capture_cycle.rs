use crate::camera::Camera;
use crate::config::UploadConfig;
use crate::fs;
use crate::observation::Observation;
use crate::sensor::Sensor;
use crate::upload;

pub fn run(
    camera: &Camera,
    sensor: &mut Sensor,
    upload_config: &UploadConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = fs::create_file_name();

    camera.capture(&file_name)?;

    println!("Saved as \"{}\"", &file_name);

    let (temperature, humidity) = sensor.read()?;

    println!("Temperature: {} ℃, Humidity: {} %RH", temperature, humidity);

    let observation = Observation {
        file_name,
        captured_at: chrono::Local::now(),
        temperature,
        humidity,
    };

    println!(
        "Observation: {} / {} ℃ / {} %RH",
        observation.file_name, observation.temperature, observation.humidity
    );

    upload::upload(&observation, &upload_config.worker_url)?;

    std::fs::remove_file(&observation.file_name)?;

    Ok(())
}
