use ::influxdb::InfluxDbWriteable;
use chrono::{DateTime, Local, Utc};
use uuid::Uuid;

#[derive(InfluxDbWriteable, serde::Deserialize, Debug)]
pub struct SoilMoistureMeasurement {
    time: DateTime<Local>,
    data: f32,
    #[influxdb(tag)]
    unit: String,
    #[influxdb(tag)]
    id: String,
    #[influxdb(tag)]
    sensor_id: String,
    #[influxdb(tag)]
    location: String,
}

impl SoilMoistureMeasurement {
    pub fn new<S: Into<String>>(
        data: f32,
        unit: S,
        sensor_id: S,
        location: S,
    ) -> SoilMoistureMeasurement {
        SoilMoistureMeasurement {
            time: Utc::now().into(),
            data,
            id: Uuid::new_v4().to_string(),
            sensor_id: sensor_id.into(),
            unit: unit.into(),
            location: location.into(),
        }
    }

    pub fn get(&self) -> (&DateTime<Local>, f32, &String, &String, &String, &String) {
        (
            &self.time,
            self.data,
            &self.unit,
            &self.id,
            &self.sensor_id,
            &self.location,
        )
    }
}
