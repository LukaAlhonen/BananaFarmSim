use std::time::Duration;

use rumqttc::v5::{
    mqttbytes::QoS, AsyncClient, ClientError, ConnectionError, Event, EventLoop, MqttOptions,
};

pub struct MqttSubscriber {
    client: AsyncClient,
    eventloop: EventLoop,
}

pub struct SubscriberParams {
    pub name: String,
    pub broker_address: String,
    pub broker_port: u16,
}

impl MqttSubscriber {
    pub fn new(
        SubscriberParams {
            name,
            broker_address,
            broker_port,
        }: SubscriberParams,
    ) -> Self {
        let mut mqttoptions = MqttOptions::new(&name, &broker_address, broker_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
        MqttSubscriber { client, eventloop }
    }

    pub async fn subscribe(&self, topic: &str) -> Result<(), ClientError> {
        self.client.subscribe(topic, QoS::AtLeastOnce).await?;

        Ok(())
    }

    pub async fn poll(&mut self) -> Result<Event, ConnectionError> {
        self.eventloop.poll().await
    }
}
