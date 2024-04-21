use error_stack::{Report, ResultExt};

use crate::error::Error;
use crate::kafka::{EventPublisher, EventSubscriber, KafkaConfig};

pub async fn setup_kafka() -> Result<(EventPublisher, EventSubscriber), Report<Error>> {
    let mut publisher = KafkaConfig::new();
    publisher.set(
        "bootstrap.servers",
        dotenvy::var("KAFKA_BOOTSTRAP").change_context_lazy(|| Error::Env {
            key: "KAFKA_BOOTSTRAP",
        })?,
    );

    let mut subscriber = publisher.clone();

    let publisher = EventPublisher::new(&mut publisher).await?;
    let subscriber = EventSubscriber::new(&mut subscriber).await?;

    Ok((publisher, subscriber))
}
