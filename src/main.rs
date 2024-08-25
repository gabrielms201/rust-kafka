mod consumer;
use consumer::kafka_consumer::KafkaConsumer;

#[tokio::main] 
async fn main() {
    let consumer = KafkaConsumer::new("rust-kafka-test", "rust-kafka", "localhost:9092");
    
    consumer.start_async().await;
}