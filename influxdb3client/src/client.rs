use reqwest::Client;

use crate::SoilMoistureMeasurement;

pub struct InfluxDB3Client {
    client: Client,
    url: String,
    token: String,
    table: String,
}

impl InfluxDB3Client {
    pub fn new<S: Into<String>>(url: S, token: S, table: S) -> Self {
        InfluxDB3Client {
            client: Client::new(),
            url: url.into(),
            token: token.into(),
            table: table.into(),
        }
    }

    pub async fn write_query(
        &self,
        measurement: SoilMoistureMeasurement,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let query = measurement.into_query_string(&self.table);
        let auth_header = format!("Bearer {}", &self.token);

        // send query
        let res = self
            .client
            .post(&self.url)
            .header("Authorization", auth_header)
            .body(query)
            .send()
            .await?;

        Ok(res.status().is_success())
    }
}
