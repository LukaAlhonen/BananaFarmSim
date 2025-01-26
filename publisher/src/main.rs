use log::{error, info};
use models::SoilMoistureMeasurement;
use rand::Rng;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use std::env;
use std::time::Duration;
use tokio::{task, time};
use uuid::Uuid;

fn generate_sensor_data(seed: f32) -> SoilMoistureMeasurement {
    let sensor_id = Uuid::new_v4();
    let mut rng = rand::thread_rng();
    let data: f32 = seed + rng.gen_range(-0.3..0.3);

    SoilMoistureMeasurement::new(data, "cb", &sensor_id.to_string())
}

#[tokio::main]
async fn main() {
    // Init env and logger
    env_logger::init();
    dotenv::from_path("publisher/.env").ok();

    // Load env vars
    let name = env::var("NAME").expect("NAME MUST BE SET");
    let broker_address = env::var("BROKER_ADDRESS").expect("BROKER_ADDRESS MUST BE SET");
    let broker_port = env::var("BROKER_PORT").expect("BROKER_PORT MUST BE SET");
    let topic = env::var("TOPIC").expect("TOPIC MUST BE SET");

    let (client, mut eventloop) = init_client(
        name,
        broker_address,
        broker_port.parse().expect("FAILED TO PARSE BROKER_PORT"),
    );
    let mut seed = 30.2;
    task::spawn(async move {
        for _i in 0..100 {
            let sensor_data = generate_sensor_data(seed);
            let (time, sensor_id, data, unit) = sensor_data.get();
            seed = data.clone();
            let message = format!(
                "{{\n  \"time\": \"{}\",\n \"sensor_id\": \"{}\",\n \"data\": {:.2},\n  \"unit\": \"{}\"\n}}",
                time, sensor_id, data, unit
            );

            match client
                .publish(&topic, QoS::AtLeastOnce, false, message.clone())
                .await
            {
                Ok(_) => info!("Published message {:?} to {}", &message, &topic),
                Err(e) => error!("Failed to publish message {:?}: {:?}", &message, e),
            }
            time::sleep(Duration::from_millis(2000)).await;
        }
    });

    info!("Finnished publishing!");

    loop {
        let event = eventloop.poll().await.unwrap();
        info!("Received = {:?}", event);
    }
}

fn init_client<S: Into<String>>(id: S, host: S, port: u16) -> (AsyncClient, EventLoop) {
    let mut mqttoptions = MqttOptions::new(id, host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    AsyncClient::new(mqttoptions, 10)
}
