mod consumer;
mod producer;
mod handler;

pub use self::{
    consumer::*,
    producer::*,
    handler::*
};

pub type KafkaConfig = rdkafka::ClientConfig;