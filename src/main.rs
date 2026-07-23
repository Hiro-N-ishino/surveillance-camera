mod camera;

fn main() -> Result<(), String> {
    camera::Camera::new()?;

    println!("Camera initialized.");

    Ok(())
}
