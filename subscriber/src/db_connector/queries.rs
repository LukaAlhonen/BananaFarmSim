use influxdb::{Client, InfluxDbWriteable};
use models::SoilMoistureMeasurement;
use std::error::Error;

pub async fn write_to_db(
    client: &Client,
    reading: SoilMoistureMeasurement,
    query: &str,
) -> Result<(), Box<dyn Error>> {
    let write_query = reading.into_query(query);

    if let Err(err) = client.query(write_query).await {
        return Err(format!("DB write failed: {}", err).into());
    }
    Ok(())
}
