use models::SoilMoistureMeasurement;
use serde_json::Error as SerdeErr;

pub fn parse_soil_measurement(payload: &str) -> Result<SoilMoistureMeasurement, SerdeErr> {
    let measurement: SoilMoistureMeasurement = serde_json::from_str(payload)?;
    Ok(measurement)
}
