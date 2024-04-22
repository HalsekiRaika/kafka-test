use error_stack::{Report, ResultExt};

use crate::error::Error;
use crate::kafka::{EventPublisher, EventSubscriber, KafkaConfig};

pub async fn setup_kafka() -> Result<(EventPublisher, EventSubscriber), Report<Error>> {
    let mut origin = KafkaConfig::new();
    origin.set(
        "bootstrap.servers",
        dotenvy::var("KAFKA_BOOTSTRAP").change_context_lazy(|| Error::Env {
            key: "KAFKA_BOOTSTRAP",
        })?,
    );
    
    let publisher = EventPublisher::new(origin.clone()).await?;
    let subscriber = EventSubscriber::new(origin).await?;

    Ok((publisher, subscriber))
}
