use error_stack::{Report, ResultExt};
use futures::StreamExt;
use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::Message;
use rdkafka::message::BorrowedMessage;
use serde::de::DeserializeOwned;
use tracing::Instrument;
use crate::error::Error;
use crate::kafka::EventSubscriber;


pub struct Subscriber<E>(tokio::sync::mpsc::UnboundedReceiver<E>);

impl<E> Subscriber<E> {
    pub async fn recv(&mut self) -> Option<E> {
        self.0.recv().await
    }
}

pub struct SubscribeHandler;

impl SubscribeHandler {
    pub async fn subscribe<E: DeserializeOwned + Send + 'static>(topic: impl AsRef<str>, subscriber: EventSubscriber) -> Result<Subscriber<E>, Report<Error>> {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        subscriber.as_ref().subscribe(&[topic.as_ref()])
            .change_context_lazy(|| Error::Kafka)?;

        tracing::trace!("subscribed topic: {}.", topic.as_ref());

        tokio::spawn(async move {
            tracing::trace!("start.");
            while let Some(payload) = subscriber.as_ref().stream().next().await {
                let payload = match payload {
                    Ok(payload) => payload,
                    Err(e) => {
                        tracing::error!("An error occurred while receiving the payload \n {}", e);
                        continue;
                    }
                };

                tracing::trace!("partition: {}, offset: {}, payload: {:?}", payload.partition(), payload.offset(), payload);

                let event: E = match procedure::<E>(&payload).await {
                    Ok(event) => event,
                    Err(e) => {
                        tracing::error!("{}", e);
                        continue;
                    }
                };

                if let Err(e) = sender.send(event) {
                    tracing::error!("{}", e);
                    break;
                }

                if let Err(e) = subscriber.as_ref().commit_message(&payload, CommitMode::Async) {
                    tracing::error!("{}", e);
                    continue;
                }

                async fn procedure<E>(msg: &BorrowedMessage<'_>) -> Result<E, Report<Error>>
                    where E: DeserializeOwned + Send,
                {
                    let event: E = msg.payload_view::<str>()
                        .transpose()
                        .change_context_lazy(|| Error::Kafka)?
                        .map(serde_json::from_str::<E>)
                        .transpose()
                        .change_context_lazy(|| Error::Serde)?
                        .ok_or(Error::Other)?;

                    Ok(event)
                }
            }
        }.instrument(tracing::trace_span!("subscriber")));

        Ok(Subscriber(receiver))
    }
}
