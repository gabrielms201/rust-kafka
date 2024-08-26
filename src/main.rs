mod consumer;
mod contracts;
use contracts::person::Person;
use consumer::kafka_consumer::AsyncCallback;
use consumer::kafka_consumer::KafkaConsumer;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

async fn person_cb(person: Person)
{
    let mut stdout = tokio::io::stdout();

    stdout.write(person.name.as_bytes()).await.unwrap();
    stdout.write(b"\n").await.unwrap();
}

//TODO: Mover para struct separada de Callback
fn create_callback<T, F, Fut>(handler: F) -> AsyncCallback<T>
where
    T: Send + Sync + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    Arc::new(move |item: T| Box::pin(handler(item)))
}

#[tokio::main] 
async fn main() {
    let callback = create_callback(person_cb);

    let consumer = KafkaConsumer::<Person>::new(
        "rust-kafka-test",
        "group",
        "localhost:9092",
        callback.clone(),
    );
    
    consumer.start_async().await;
}