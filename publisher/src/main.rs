mod sensor;

use rand::Rng;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use sensor::SensorData;
use std::time::Duration;
use tokio::{task, time};

fn generate_sensor_data(seed: f32) -> SensorData {
    let mut rng = rand::thread_rng();
    let data: f32 = seed + rng.gen_range(-0.3..0.3);

    SensorData::new(data, String::from("cb"))
}

#[tokio::main]
async fn main() {
    // Define MQTT options
    let mut mqttoptions = MqttOptions::new("pub-1", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) =
        init_client(String::from("pub-1"), String::from("localhost"), 1883);
    let mut seed = 30.2;
    task::spawn(async move {
        for _i in 0..100 {
            let sensor_data = generate_sensor_data(seed);
            let (timestamp, data, unit) = sensor_data.get();
            seed = data.clone();
            let message = format!(
                "{{\n  \"timestamp\": {},\n  \"data\": {:.2},\n  \"unit\": \"{}\"\n}}",
                timestamp, data, unit
            );

            match client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, message.clone())
                .await
            {
                Ok(_) => println!("Published message {:?} to hello/rumqtt", message.clone()),
                Err(e) => eprintln!("Failed to publish message {:?}: {:?}", message.clone(), e),
            }
            time::sleep(Duration::from_millis(2000)).await;
        }
    });

    println!("Finnished publishing!");

    loop {
        let event = eventloop.poll().await.unwrap();
        println!("Received = {:?}", event);
    }
}

fn init_client(id: String, host: String, port: u16) -> (AsyncClient, EventLoop) {
    let mut mqttoptions = MqttOptions::new(id, host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    AsyncClient::new(mqttoptions, 10)
}
