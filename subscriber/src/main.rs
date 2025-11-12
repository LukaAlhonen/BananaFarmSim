use influxdb3client::{InfluxDB3Client, SoilMoistureMeasurement};
use log::{error, info};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::env;
use std::process;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Init logger
    env_logger::init();

    // Load env
    dotenv::from_path("subscriber/.env").ok();

    // Db env vars
    let token = env::var("INFLUXDB_TOKEN").expect("DB_TOKEN MUST BE SET");
    let db_address = env::var("DB_ADDRESS").expect("DB_ADDRESS MUST BE SET");
    let table = env::var("TABLE").expect("TABLE MUST BE SET");

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
            error!("Error Subscribing: {:?}", e);
            process::exit(1);
        }
    }

    let db_client = InfluxDB3Client::new(db_address, token, table);

    loop {
        let notification = eventloop.poll().await;
        match notification {
            // If message recieved on subscribed topic -> write to db
            Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish))) => {
                if let Ok(message) = String::from_utf8(publish.payload.to_vec()) {
                    match SoilMoistureMeasurement::from_payload(&message) {
                        // write message to database
                        // TODO: implement some sort of backoff, so if write was not successfull,
                        // try again. Should implement custom error type for client so I can know
                        // if I should try again or not
                        Ok(parsed_message) => match db_client.write_query(parsed_message).await {
                            Ok(_) => info!("Message written to database"),
                            Err(err) => error!("Error writing to db: {}", err),
                        },
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
