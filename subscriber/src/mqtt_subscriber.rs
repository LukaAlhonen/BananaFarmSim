use log::{error, info};
use std::{process, time::Duration};

use influxdb3client::{InfluxDB3Client, SoilMoistureMeasurement};
use rumqttc::v5::{mqttbytes::QoS, AsyncClient, EventLoop, MqttOptions};

pub struct MqttSubscriber {
    client: AsyncClient,
    eventloop: EventLoop,
    topic: String,
    db_client: InfluxDB3Client,
}

pub struct SubscriberParams {
    pub topic: String,
    pub name: String,
    pub broker_address: String,
    pub broker_port: u16,
    pub db_client: InfluxDB3Client,
}

impl MqttSubscriber {
    pub fn new(
        SubscriberParams {
            topic,
            name,
            broker_address,
            broker_port,
            db_client,
        }: SubscriberParams,
    ) -> Self {
        let mut mqttoptions = MqttOptions::new(&name, &broker_address, broker_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
        MqttSubscriber {
            client,
            eventloop,
            topic,
            db_client,
        }
    }

    pub async fn subscribe(&mut self) {
        // Init logger
        env_logger::init();

        match self.client.subscribe(&self.topic, QoS::AtLeastOnce).await {
            Ok(_) => {}
            Err(e) => {
                error!("Error Subscribing: {:?}", e);
                process::exit(1);
            }
        }

        loop {
            let notification = self.eventloop.poll().await;
            match notification {
                // If message recieved on subscribed topic -> write to db
                Ok(rumqttc::v5::Event::Incoming(rumqttc::v5::mqttbytes::v5::Packet::Publish(
                    publish,
                ))) => {
                    if let Ok(message) = String::from_utf8(publish.payload.to_vec()) {
                        match SoilMoistureMeasurement::from_payload(&message) {
                            // write message to database
                            // TODO: implement some sort of backoff, so if write was not successfull,
                            // try again. Should implement custom error type for client so I can know
                            // if I should try again or not
                            Ok(parsed_message) => {
                                match self.db_client.write_query(parsed_message).await {
                                    Ok(_) => info!("Message written to database"),
                                    Err(err) => error!("Error writing to db: {}", err),
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
}
