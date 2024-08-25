use tokio::io::AsyncWriteExt;
use rdkafka::consumer::{Consumer, StreamConsumer};
use serde::de::DeserializeOwned;
use rdkafka::{ClientConfig, Message};

pub struct KafkaConsumer<T> where T: DeserializeOwned,
{
    topic: String,
    consumer_group: String,
    bootstrap_server: String,
    stream_consumer: StreamConsumer,
    _marker: std::marker::PhantomData<T>,
}

impl<T> KafkaConsumer<T> where T: DeserializeOwned{
    pub fn new(topic: &str, consumer_group: &str, bootstrap_server: &str) -> Self {
        let stream_consumer = ClientConfig::new()
                        .set("bootstrap.servers", bootstrap_server)
                        .set("enable.partition.eof", "false")
                        .set("group.id", "rust-kafka")
                        .create()
                        .expect("Failed to create client");

        return KafkaConsumer {
            topic: topic.to_string(),
            consumer_group: consumer_group.to_string(),
            bootstrap_server: bootstrap_server.to_string(),
            stream_consumer,
            _marker: std::marker::PhantomData,
        };
    }

    pub async fn start_async(&self){
        self.stream_consumer.subscribe(&[&self.topic]).unwrap();

        let mut stdout = tokio::io::stdout();

        loop {
            stdout.write(b"> ").await.unwrap();
            stdout.flush().await.unwrap();
         
            tokio::select! {
                message = self.stream_consumer.recv() => {
                    let message = message.expect("Failed to read message").detach();
                    
                    let payload = message.payload().unwrap();
                    let payload_str = std::str::from_utf8(payload);
                    let person: T = serde_json::from_str(payload_str.unwrap()).unwrap();

                    stdout.write(b"CONFIA").await.unwrap();
                    stdout.write(b"\n").await.unwrap();
                }
            }
        }
    }
}