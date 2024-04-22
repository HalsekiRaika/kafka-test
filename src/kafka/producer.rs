use error_stack::{Report, ResultExt};
use rdkafka::client::DefaultClientContext;
use rdkafka::config::FromClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use serde::Serialize;
use tracing::Instrument;
use uuid::Uuid;
use crate::error::Error;
use crate::kafka::KafkaConfig;
use crate::runtime::TokioRuntime;

pub struct EventPublisher(FutureProducer<DefaultClientContext, TokioRuntime>);

impl EventPublisher {
    #[tracing::instrument(skip(config), name = "EventPublisher Setup")]
    pub async fn new(mut config: KafkaConfig) -> Result<Self, Report<Error>> {
        config.set("message.timeout.ms", "5000");
        
        let producer = tokio::task::spawn_blocking(move || {
            FutureProducer::from_config(&config).change_context_lazy(|| Error::Kafka)
        })
            .instrument(tracing::info_span!("producer-create"))
            .await
            .change_context_lazy(|| Error::Kafka)??;
        
        Ok(Self(producer))
    }

    pub async fn publish<E>(&self, topic: impl AsRef<str>, key: impl AsRef<Uuid>, event: &E) -> Result<(), Report<Error>>
        where E: Serialize + Sync + Send + 'static
    {
        let message = serde_json::to_string(event).change_context_lazy(|| Error::Serde)?;

        tracing::trace!("serialized {}", message);

        let record = FutureRecord::to(topic.as_ref())
            .key(key.as_ref().as_bytes())
            .payload(&message);
        self.0.send(record, Timeout::Never)
            .await
            .map_err(|e| {
                tracing::error!("{} <message>: {:?}", e.0, e.1);
                e.0
            })
            .change_context_lazy(|| Error::Kafka)?;

        tracing::trace!("sent : {}", message);
        Ok(())
    }
}
