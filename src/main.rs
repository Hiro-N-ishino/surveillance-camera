mod camera;

fn main() -> Result<(), String> {
    println!("");
    println!("== Main Process Start ==");
    println!("");

    let c = camera::Camera::new(1920, 1080)?;
    let f_name = camera::Camera::create_file_name();

    println!("Camera initialized.");

    camera::Camera::print_info(&c);
    c.capture(&f_name)?;
    println!("Saved as \"{}\"", &f_name);

    println!("");
    println!("== Main Process End ==");
    println!("");
    Ok(())
}
