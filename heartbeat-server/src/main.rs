use actix_web::{web, App, HttpServer, Responder};
use rumqttc::{MqttOptions, AsyncClient};
use tokio::select;
use tokio::time::{self, Duration};

mod config;

async fn index() -> impl Responder {
    "Hello from Actix-web with MQTT!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = config::get();

    let options = MqttOptions::new("test_client", "localhost", 1883);
    let ( client, mut event_loop) = AsyncClient::new(options, 10);

    // Subscribe to a topic
    client.subscribe("test/topic", rumqttc::QoS::AtLeastOnce).await.unwrap();

    // Use a separate asynchronous task to poll the event loop
    let mqtt_task = tokio::spawn(async move {
        loop {
            select! {
                notification = event_loop.poll() => match notification {
                    Ok(notification) => println!("Received: {:?}", notification),
                    Err(e) => eprintln!("Error polling MQTT event loop: {:?}", e),
                },
                _ = time::sleep(Duration::from_secs(1)) => {
                    // Optional: handle periodic tasks or keep-alive messages
                },
            }
        }
    });

    // Actix-web server setup
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    mqtt_task.await?;

    Ok(())
}
