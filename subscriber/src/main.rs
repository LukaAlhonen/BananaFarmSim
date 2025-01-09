use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::process;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("sub-1", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    match client.subscribe("hello/rumqtt", QoS::AtLeastOnce).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {:?}", e);
            process::exit(1);
        }
    }

    loop {
        let notification = eventloop.poll().await;
        match notification {
            Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish))) => {
                let message = String::from_utf8(publish.payload.to_vec()).unwrap();
                println!("Received message topic '{}': {}", publish.topic, message);
            }
            Ok(_) => {
                println!("{:?}", notification);
            }
            Err(e) => {
                eprintln!("Connecion error: {:?}", e);
                break;
            }
        }
    }
}
