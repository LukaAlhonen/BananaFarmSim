use rand::Rng;
use uuid::Uuid;

// Represent a soil moisture sensor
pub struct Sensor {
    seed: f32,
    id: Uuid,
}

impl Sensor {
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        Sensor { seed: 30.0, id }
    }

    // seed sensor
    pub fn _seed(&mut self, seed: f32) {
        self.seed = seed;
    }

    // read from sensor
    pub fn read(&self) -> f32 {
        let mut rng = rand::thread_rng();
        &self.seed + rng.gen_range(-0.3..0.3)
    }

    // get sensor_id
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    // get sensor_id as string
    pub fn get_id_string(&self) -> String {
        self.id.to_string()
    }
}
