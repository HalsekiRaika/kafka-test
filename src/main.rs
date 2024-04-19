mod error;
mod event;
mod setup;
mod kafka;

use error_stack::Report;
use tracing_subscriber::{layer::SubscriberExt, Layer, util::SubscriberInitExt};
use uuid::Uuid;
use crate::error::Error;
use crate::event::Event;
use crate::kafka::SubscribeHandler;
use crate::setup::setup_kafka;


#[tokio::main]
async fn main() -> Result<(), Report<Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_filter(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "kafka_test=trace,librdkafka=trace,rdkafka::client=debug".into())))
            .with_filter(tracing_subscriber::filter::LevelFilter::TRACE))
        .init();

    let (publisher, subscriber) = setup_kafka()?;

    for _ in 0..5 {
        let id = Uuid::new_v4();
        let event = Event::Created { id };
        publisher.publish("event", &id, &event).await?;
    }

    let mut subscriber = SubscribeHandler::subscribe::<Event>("event", subscriber).await?;

    while let Some(ev) = subscriber.recv().await {
        tracing::trace!("{:?}", ev);
    }

    Ok(())
}