mod error;
mod event;
mod setup;
mod kafka;
mod runtime;

use error_stack::Report;
use tracing_subscriber::{layer::SubscriberExt, Layer, util::SubscriberInitExt};
use uuid::Uuid;
use crate::error::Error;
use crate::event::Event;
use crate::kafka::SubscribeHandler;
use crate::setup::setup_kafka;


#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Report<Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_filter(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "kafka_test=trace,librdkafka=trace,rdkafka=trace".into())))
            .with_filter(tracing_subscriber::filter::LevelFilter::TRACE))
        .init();

    let (publisher, subscriber) = setup_kafka().await?;

    for _ in 0..5 {
        let id = Uuid::new_v4();
        let event = Event::Created { id };
        publisher.publish("event", &id, &event).await?;
    }

    let mut subscriber = SubscribeHandler::new(subscriber).subscribe::<Event>("event").await?;

    while let Some(ev) = subscriber.recv().await {
        tracing::trace!("{:?}", ev);
    }

    Ok(())
}