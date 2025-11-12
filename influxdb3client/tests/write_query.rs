#[cfg(test)]
mod tests {
    use influxdb3client::{InfluxDB3Client, SoilMoistureMeasurement};

    #[tokio::test]
    async fn test_write_query_to_db() {
        // OBS! local test db needs to be running for this test to work
        let client = InfluxDB3Client::new(
            "http://localhost:8182/api/v3/write_lp?db=test_db",
            "no_auth",
            "soil_moisture_readings",
        );

        let measurement = SoilMoistureMeasurement::new(30.0, "cb", "sensor_01", "location_01");

        let res = client.write_query(measurement).await.unwrap();

        assert!(res);
    }
}
