pub mod mqtt_subscriber;

use influxdb3client::InfluxDB3Client;
use mqtt_subscriber::{MqttSubscriber, SubscriberParams};
use std::env;

#[tokio::main]
async fn main() {
    // Load env
    dotenv::from_path("subscriber/.env").ok();

    // Db env vars
    let token = env::var("INFLUXDB_TOKEN").expect("DB_TOKEN MUST BE SET");
    let db_address = env::var("DB_ADDRESS").expect("DB_ADDRESS MUST BE SET");
    let table = env::var("TABLE").expect("TABLE MUST BE SET");

    // Rumqtt env vars
    let name = env::var("NAME").expect("NAME MUST BE SET");
    let topic = env::var("TOPIC").expect("TOPIC MUST BE SET");
    let broker_address = env::var("BROKER_ADDRESS").expect("BROKER_ADDRESS MUST BE SET");
    let broker_port = env::var("BROKER_PORT").expect("BROKER_PORT MUST BE SET");

    let db_client = InfluxDB3Client::new(db_address, token, table);
    let mut client = MqttSubscriber::new(SubscriberParams {
        topic,
        name,
        broker_address,
        broker_port: broker_port.parse().expect("Failed to parse broker port"),
        db_client,
    });

    client.subscribe().await;
}
