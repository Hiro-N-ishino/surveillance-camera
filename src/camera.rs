use std::process::Command;

pub struct Camera {
    width: u32,
    height: u32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let success = true;
        if success {
            Ok(Self { width, height })
        } else {
            Err(String::from("Error"))
        }
    }

    pub fn print_info(&self) {
        println!("Resolution: {} x {}", self.width, self.height);
    }

    pub fn capture(&self, file_name: &str) -> Result<(), String> {
        let success = true;

        if success {
            let output = Command::new("echo")
                .arg("Hello")
                .arg("Camera")
                .output()
                .map_err(|e| e.to_string())?;

            //println!("{:?}", output);
            //println!("{}", String::from_utf8(output.stdout).unwrap());
            println!(
                "{}",
                String::from_utf8(output.stdout).map_err(|e| e.to_string())?
            );

            Ok(())
        } else {
            Err(String::from("Error"))
        }
    }
}
