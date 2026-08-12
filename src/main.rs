mod camera;
mod capture_cycle;
mod config;
mod fs;
mod upload;

use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Main Process Start ==");

    let mut consecutive_error_count: u32 = 0;
    let conf = config::Config::load("config.toml")?;
    let c = camera::Camera::new(&conf.camera)?;

    c.print_info();

    loop {
        match capture_cycle::run(&c, &conf.upload) {
            Ok(_) => {
                consecutive_error_count = 0;
            }
            Err(e) => {
                eprintln!("Capture cycle error: \"{}\"", e);
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
