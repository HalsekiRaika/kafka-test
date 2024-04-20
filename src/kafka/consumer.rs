use std::time::Duration;
use error_stack::{Report, ResultExt};
use rdkafka::ClientContext;
use rdkafka::consumer::{ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::util::Timeout;
use crate::error::Error;
use crate::kafka::KafkaConfig;

pub struct EventSubscriber(StreamConsumer<SubscribeContext>);

impl EventSubscriber {
    pub fn new(config: &mut KafkaConfig) -> Result<Self, Report<Error>> {
        config.set("group.id", "my-group-1");
        config.set("enable.partition.eof", "false");
        config.set("session.timeout.ms", "550");
        config.set("max.poll.interval.ms", "600");
        config.set("enable.auto.commit", "false");
        config.set("enable.auto.offset.store", "false");
        config.set("auto.offset.reset", "latest");
        config.set("debug", "consumer");
        Ok(Self(config.create_with_context(SubscribeContext).change_context_lazy(|| Error::Kafka)?))
    }
}

impl AsRef<StreamConsumer<SubscribeContext>> for EventSubscriber {
    fn as_ref(&self) -> &StreamConsumer<SubscribeContext> {
        &self.0
    }
}

pub struct SubscribeContext;

impl ClientContext for SubscribeContext {}

impl ConsumerContext for SubscribeContext {
    #[tracing::instrument(skip(self, rebalance), name = "pre-rebalance")]
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        tracing::trace!("{:?}", rebalance);
    }

    #[tracing::instrument(skip(self, rebalance), name = "post-rebalance")]
    fn post_rebalance(&self, rebalance: &Rebalance) {
        tracing::trace!("{:?}", rebalance);
    }

    #[tracing::instrument(skip(self), name = "poll")]
    fn main_queue_min_poll_interval(&self) -> Timeout {
        Timeout::After(Duration::from_millis(500))
    }
}
