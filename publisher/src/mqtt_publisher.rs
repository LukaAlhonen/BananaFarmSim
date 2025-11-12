use rumqttc::v5::{
    mqttbytes::QoS, AsyncClient, ClientError, ConnectionError, Event, EventLoop, MqttOptions,
};
use std::time::Duration;

// Represents a rumqtt AsyncClient that acts as a publisher
pub struct MqttPublisher {
    client: AsyncClient,
    eventloop: EventLoop,
}

impl MqttPublisher {
    pub fn new<S: Into<String>>(name: S, broker_address: S, broker_port: u16) -> Self {
        let mut mqttoptions = MqttOptions::new(name.into(), broker_address.into(), broker_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
        MqttPublisher {
            client: client,
            eventloop: eventloop,
        }
    }

    // publish new message to a chosen topic
    pub async fn publish<S: Into<String>>(&self, topic: S, message: S) -> Result<(), ClientError> {
        self.client
            .publish(topic.into(), QoS::AtLeastOnce, false, message.into())
            .await
    }

    // read a message from the eventloop
    pub async fn poll(&mut self) -> Result<Event, ConnectionError> {
        self.eventloop.poll().await
    }
}
