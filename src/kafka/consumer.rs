use std::sync::Arc;
use std::time::Duration;
use error_stack::{Report, ResultExt};
use rdkafka::ClientContext;
use rdkafka::config::FromClientConfigAndContext;
use rdkafka::consumer::{ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::util::Timeout;
use crate::error::Error;
use crate::kafka::KafkaConfig;
use crate::runtime::TokioRuntime;

pub struct EventSubscriber(Arc<StreamConsumer<SubscribeContext, TokioRuntime>>);

impl EventSubscriber {
    #[tracing::instrument(skip(config), name = "EventSubscriber Setup")]
    pub async fn new(config: &mut KafkaConfig) -> Result<Self, Report<Error>> {
        config.set("group.id", "my-group-1");
        config.set("enable.partition.eof", "false");
        config.set("session.timeout.ms", "5500");
        config.set("max.poll.interval.ms", "6000");
        config.set("enable.auto.commit", "false");
        config.set("enable.auto.offset.store", "false");
        config.set("debug", "consumer");
        let consumer = StreamConsumer::from_config_and_context(config, SubscribeContext)
            .change_context_lazy(|| Error::Kafka)?;
        Ok(Self(Arc::new(consumer)))
    }
}

impl Clone for EventSubscriber {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl AsRef<StreamConsumer<SubscribeContext, TokioRuntime>> for EventSubscriber {
    fn as_ref(&self) -> &StreamConsumer<SubscribeContext, TokioRuntime> {
        &self.0.as_ref()
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
