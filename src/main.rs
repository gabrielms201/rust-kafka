mod consumer;
mod contracts;
use contracts::person::Person;
use consumer::kafka_consumer::KafkaConsumer;
use std::sync::Arc;
use futures::future::BoxFuture;
use tokio::io::AsyncWriteExt;



async fn person_cb(person: Person)
{
    let mut stdout = tokio::io::stdout();

    stdout.write(person.name.as_bytes()).await.unwrap();
    stdout.write(b"\n").await.unwrap();
}

fn create_callback() -> impl Fn(Person) -> BoxFuture<'static, ()> + Send + Sync {
    move |person: Person| -> BoxFuture<'static, ()> {
        Box::pin(person_cb(person))
    }
}

#[tokio::main] 
async fn main() {
    let callback: Arc<dyn Fn(Person) -> BoxFuture<'static, ()> + Send + Sync> = Arc::new(create_callback());

    let consumer = KafkaConsumer::<Person>::new(
        "rust-kafka-test",
        "group",
        "localhost:9092",
        callback.clone(),
    );
    
    consumer.start_async().await;
}