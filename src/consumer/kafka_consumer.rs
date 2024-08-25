use tokio::io::AsyncWriteExt;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};


pub struct KafkaConsumer {
    topic: String,
    consumer_group: String,
    bootstrap_server: String,
    stream_consumer: StreamConsumer,
}

impl KafkaConsumer{
    pub fn new(topic: &str, consumer_group: &str, bootstrap_server: &str) -> KafkaConsumer {
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
            stream_consumer
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
                    stdout.write(payload).await.unwrap();
                    stdout.write(b"\n").await.unwrap();
                }
            }
        }
    }
}