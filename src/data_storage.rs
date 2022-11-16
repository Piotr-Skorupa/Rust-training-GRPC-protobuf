use sensor::SensorData;

pub mod sensor {
    include!("../tonic_generated/sensor.rs");
}

#[derive(Default)]
pub struct DataStorage {
    last_packages_: Vec<SensorData>,
}

impl DataStorage {
    pub fn add(&mut self, data: SensorData) {
        self.last_packages_.push(data);
    }

    pub fn get_packages(&self, count: usize) -> Vec<SensorData> {
        if self.last_packages_.len() >= count {
            return (&self.last_packages_[(self.last_packages_.len() - count)..]).to_vec();
        }

        self.last_packages_.clone()
    }
}