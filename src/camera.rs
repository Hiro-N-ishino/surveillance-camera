pub struct Camera {}

impl Camera {
    pub fn new() -> Result<Self, String> {
        let success = true;
        if success {
            Ok(Self {})
        } else {
            Err(String::from("Error"))
        }
    }
}
