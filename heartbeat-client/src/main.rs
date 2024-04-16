use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::time::{self, Duration};
use std::sync::Arc;

mod config;

#[tokio::main]
async fn main() {
    let conf = config::get();


    let options = MqttOptions::new(conf.mqtt_topic, conf.mqtt_host, conf.mqtt_port.try_into().unwrap());
    let (client, mut event_loop) = AsyncClient::new(options, 10);

    


    // tokio::spawn(async move {
    //     loop {
    //         client.publish("test/topic", QoS::AtLeastOnce, false, "hi!").await.unwrap();
    //         time::sleep(Duration::from_secs(10)).await;
    //     }
    // });

    // loop {
    //     event_loop.poll().await.unwrap();
    //     tokio::time::sleep(Duration::from_secs(1)).await;
    // }
}