use rdkafka::consumer::{Consumer, StreamConsumer};
use serde::de::DeserializeOwned;
use rdkafka::{ClientConfig, Message};
use ::futures::future::BoxFuture;
use std::sync::Arc;

pub(crate) type AsyncCallback<I> = dyn 'static + Send + Sync + Fn(I) -> BoxFuture<'static, ()>;

pub struct KafkaConsumer<T>
where
    T: DeserializeOwned
{
    topic: String,
    stream_consumer: StreamConsumer,
    _marker: std::marker::PhantomData<T>,
    callback: Arc<AsyncCallback<T>>
}

impl<T> KafkaConsumer<T>
where
    T: DeserializeOwned,
{
    pub fn new(
        topic: &str,
        consumer_group: &str,
        bootstrap_server: &str,
        callback: Arc<AsyncCallback<T>>) -> Self {
        let stream_consumer = ClientConfig::new()
                        .set("bootstrap.servers", bootstrap_server)
                        .set("enable.partition.eof", "false")
                        .set("group.id", consumer_group)
                        .create()
                        .expect("Failed to create client");

        return KafkaConsumer {
            topic: topic.to_string(),
            stream_consumer,
            _marker: std::marker::PhantomData,
            callback
        };
    }

    pub async fn start_async(&self){
        self.stream_consumer.subscribe(&[&self.topic]).unwrap();

        loop {
            tokio::select! {
                message = self.stream_consumer.recv() => {
                    let message = message.expect("Failed to read message").detach();
                    
                    // TODO: Melhorar
                    let payload = message.payload().unwrap();

                    //TODO: Melhorar
                    let payload_str = std::str::from_utf8(payload);

                    let object: T = serde_json::from_str(payload_str.unwrap()).unwrap();
                    let callback = self.callback.clone();

                    callback(object).await;
                }
            }
        }
    }
}