mod camera;
mod config;
mod fs;

use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Main Process Start ==");

    let mut consecutive_error_count: u32 = 0;
    let conf = config::Config::load("config.toml")?;
    let c = camera::Camera::new(&conf.camera)?;

    c.print_info();

    loop {
        let f_name = fs::create_file_name();

        match c.capture(&f_name) {
            Ok(_) => {
                println!("Saved as \"{}\"", &f_name);
                consecutive_error_count = 0;
            }
            Err(e) => {
                eprintln!("Capture error: \"{}\"", e);
                consecutive_error_count += 1;

                if consecutive_error_count >= conf.capture.max_consecutive_errors {
                    eprintln!("Too many consecutive errors. Exiting...");
                    break;
                }
            }
        }

        thread::sleep(conf.capture.interval_duration());
    }

    Ok(())
}
