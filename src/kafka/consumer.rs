use error_stack::{Report, ResultExt};
use rdkafka::consumer::StreamConsumer;
use crate::error::Error;
use crate::kafka::KafkaConfig;

pub struct EventSubscriber(StreamConsumer);

impl EventSubscriber {
    pub fn new(config: &mut KafkaConfig) -> Result<Self, Report<Error>> {
        config.set("group.id", "my-group-1");
        config.set("enable.partition.eof", "false");
        config.set("session.timeout.ms", "550");
        config.set("max.poll.interval.ms", "600");
        config.set("enable.auto.commit", "false");
        config.set("debug", "consumer");
        Ok(Self(config.create().change_context_lazy(|| Error::Kafka)?))
    }
}

impl AsRef<StreamConsumer> for EventSubscriber {
    fn as_ref(&self) -> &StreamConsumer {
        &self.0
    }
}