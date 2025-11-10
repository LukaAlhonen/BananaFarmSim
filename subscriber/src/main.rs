mod db_connector;

use db_connector::queries::write_to_db;
use db_connector::serializers::parse_soil_measurement;
use db_connector::DbClient;
use log::{error, info};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::env;
use std::process;
use std::time::Duration;

fn backoff() {
    info!("Backing off");
}

#[tokio::main]
async fn main() {
    // Init logger
    env_logger::init();

    // Load env
    dotenv::from_path("subscriber/.env").ok();

    // Db env vars
    let token = env::var("DB_TOKEN").expect("DB_TOKEN MUST BE SET");
    let db_address = env::var("DB_ADDRESS").expect("DB_ADDRESS MUST BE SET");
    let bucket = env::var("BUCKET").expect("BUCKET MUST BE SET");
    let query = env::var("QUERY").expect("QUERY MUST BE SET");

    // Rumqtt env vars
    let sub_name = env::var("NAME").expect("NAME MUST BE SET");
    let topic = env::var("TOPIC").expect("TOPIC MUST BE SET");
    let broker_address = env::var("BROKER_ADDRESS").expect("BROKER_ADDRESS MUST BE SET");
    let broker_port = env::var("BROKER_PORT").expect("BROKER_PORT MUST BE SET");

    let mut mqttoptions = MqttOptions::new(
        sub_name,
        broker_address,
        broker_port.parse().expect("Failed to parse BROKER_PORT"),
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    match client.subscribe(topic, QoS::AtLeastOnce).await {
        Ok(_) => {}
        Err(e) => {
            error!("Error: {:?}", e);
            process::exit(1);
        }
    }

    // Init db connector
    let db_client = DbClient::new(db_address, bucket, token);

    loop {
        let notification = eventloop.poll().await;
        match notification {
            // If message recieved on subscribed topic -> write to db
            Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish))) => {
                if let Ok(message) = String::from_utf8(publish.payload.to_vec()) {
                    match parse_soil_measurement(&message) {
                        Ok(parsed_message) => {
                            match write_to_db(&db_client.client, parsed_message, &query).await {
                                Ok(_) => info!("Message written to db"),
                                Err(err) => {
                                    info!("Error writing to db: {}", err);
                                    backoff();
                                }
                            }
                        }
                        Err(err) => error!("Error parsing message: {}", err),
                    }
                } else {
                    error!("Error decoding message payload as utf-8");
                }
            }
            Ok(_) => {
                info!("{:?}", notification);
            }
            Err(e) => {
                error!("Connecion error: {:?}", e);
                break;
            }
        }
    }
}
