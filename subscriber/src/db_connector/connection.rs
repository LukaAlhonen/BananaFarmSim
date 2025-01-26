use influxdb::Client;

pub struct DbClient {
    pub client: Client,
}

impl DbClient {
    pub fn new<S: Into<String>>(db_address: S, bucket: S, token: S) -> Self {
        let client = Client::new(db_address.into(), bucket.into()).with_token(token);
        Self { client }
    }
}
