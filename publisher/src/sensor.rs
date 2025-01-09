use chrono::Local;

#[derive(Debug)]
pub struct SensorData {
    timestamp: String,
    data: f32,
    unit: String,
}

impl SensorData {
    pub fn new(data: f32, unit: String) -> SensorData {
        // Create timestamp in format Y-m-d H:M:S
        let now = Local::now();
        let formatted_time = now.format("%H:%M:%S").to_string();
        let formatted_date = now.format("%d-%m-%Y").to_string();
        let timestamp = format!("{} {}", formatted_time, formatted_date);

        SensorData {
            timestamp,
            data,
            unit,
        }
    }

    pub fn get(&self) -> (&String, f32, &String) {
        (&self.timestamp, self.data, &self.unit)
    }
}
