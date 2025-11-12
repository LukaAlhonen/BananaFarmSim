use clap::Parser;
use influxdb3client::SoilMoistureMeasurement;
use log::{error, info};
use rand::Rng;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use std::env;
use std::time::Duration;
use tokio::{task, time};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    name: String,
    #[arg(long)]
    sensor_id: String,
    #[arg(long)]
    location: String,
}

fn generate_sensor_data(seed: f32, sensor_id: &str, location: &str) -> SoilMoistureMeasurement {
    let mut rng = rand::thread_rng();
    let data: f32 = seed + rng.gen_range(-0.3..0.3);

    SoilMoistureMeasurement::new(data, "cb", sensor_id, location)
}

fn init_client<S: Into<String>>(id: S, host: S, port: u16) -> (AsyncClient, EventLoop) {
    let mut mqttoptions = MqttOptions::new(id, host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    AsyncClient::new(mqttoptions, 10)
}

#[tokio::main]
async fn main() {
    // Take publisher name, sensor_id and location as command line args
    let args = Args::parse();
    let sensor_id = args.sensor_id;
    let name = args.name;
    let location = args.location;

    // Init env and logger
    env_logger::init();
    dotenv::from_path("publisher/.env").ok();

    // Load env vars
    let broker_address = env::var("BROKER_ADDRESS").expect("BROKER_ADDRESS MUST BE SET");
    let broker_port = env::var("BROKER_PORT").expect("BROKER_PORT MUST BE SET");
    let topic = env::var("TOPIC").expect("TOPIC MUST BE SET");

    let (client, mut eventloop) = init_client(
        name,
        broker_address,
        broker_port.parse().expect("FAILED TO PARSE BROKER_PORT"),
    );

    let seed = 30.2_f32;
    task::spawn(async move {
        loop {
            let measurement: SoilMoistureMeasurement =
                generate_sensor_data(seed, sensor_id.as_str(), location.as_str());

            match measurement.into_payload() {
                Ok(message) => {
                    match client
                        .publish(&topic, QoS::AtLeastOnce, false, message.clone())
                        .await
                    {
                        Ok(_) => info!("Published message to {}", &topic),
                        Err(e) => error!("Failed to publish message {:?}: {:?}", &message, e),
                    }
                    time::sleep(Duration::from_millis(60000)).await;
                }
                Err(err) => error!("Error parsing measurement: {}", err),
            }
        }
    });

    loop {
        match eventloop.poll().await {
            Ok(event) => info!("Received: {:?}", event),
            Err(err) => error!("Connection error: {}", err),
        }
    }
}
