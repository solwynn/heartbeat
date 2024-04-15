use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::time::{self, Duration};

mod config;

#[tokio::main]
async fn main() {
    let options = MqttOptions::new("client", "localhost", 1883);
    let (client, mut event_loop) = AsyncClient::new(options, 10);

    config::check();

    tokio::spawn(async move {
        loop {
            client.publish("test/topic", QoS::AtLeastOnce, false, "hi!").await.unwrap();
            time::sleep(Duration::from_secs(10)).await;
        }
    });

    loop {
        event_loop.poll().await.unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}