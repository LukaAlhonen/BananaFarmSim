#[cfg(test)]
mod tests {
    use influxdb3client::{InfluxDB3Client, SoilMoistureMeasurement};

    #[tokio::test]
    async fn test_write_query_to_db() {
        let client = InfluxDB3Client::new(
            "http://ubuntubox.local:8181/api/v3/write_lp?db=bananafarm_test",
            "apiv3_tT5QvrwyjX7JVslrwfvJkZ-OqQVaQINPyEazla9S-DMM1xdSkGkZFN58nLMCeiIyV82eR0x42Jl436fq3IKl-Q",
            "soil_moisture_readings",
        );

        let measurement = SoilMoistureMeasurement::new(30.0, "cb", "sensor_01", "location_01");

        let res = client.write_query(measurement).await.unwrap();

        assert!(res);
    }
}
