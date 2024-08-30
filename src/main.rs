mod consumer;
mod contracts;
pub mod callbacks;
use contracts::person::Person;
use consumer::kafka_consumer::KafkaConsumer;
use tokio::io::AsyncWriteExt;
use callbacks::async_callback::create_consumer_callback;

async fn person_cb(person: Person)
{
    let mut stdout = tokio::io::stdout();

    stdout.write(person.name.as_bytes()).await.unwrap();
    stdout.write(b"\n").await.unwrap();
}

#[tokio::main] 
async fn main() {
    let consumer = KafkaConsumer::<Person>::new(
        "rust-kafka-test",
        "group",
        "localhost:9092",
        create_consumer_callback(person_cb),
    );
    
    consumer.start_async().await;
}