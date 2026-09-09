use dht_mmap_rust::{Dht, DhtType};

pub struct Sensor {
    dht: Dht,
}

impl Sensor {
    pub fn new() -> Result<Self, String> {
        let dht =
            Dht::new(DhtType::Dht11, 4).map_err(|_| "Failed to initialize DHT11".to_string())?;

        Ok(Self { dht })
    }

    pub fn read(&mut self) -> Result<(f32, f32), String> {
        let reading = self
            .dht
            .read()
            .map_err(|_| "Failed to read DHT11".to_string())?;

        Ok((reading.temperature(), reading.humidity()))
    }
}
