use ::influxdb::InfluxDbWriteable;
use chrono::{DateTime, Local, Utc};

#[derive(InfluxDbWriteable, serde::Deserialize)]
pub struct SoilMoistureMeasurement {
    time: DateTime<Local>,
    sensor_id: String,
    data: f32,
    unit: String,
}

impl SoilMoistureMeasurement {
    pub fn new<S: Into<String>>(data: f32, unit: S, sensor_id: S) -> SoilMoistureMeasurement {
        SoilMoistureMeasurement {
            time: Utc::now().into(),
            sensor_id: sensor_id.into(),
            data,
            unit: unit.into(),
        }
    }

    pub fn get(&self) -> (&DateTime<Local>, &String, f32, &String) {
        (&self.time, &self.sensor_id, self.data, &self.unit)
    }
}
