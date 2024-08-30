
use ::futures::future::BoxFuture;
use std::sync::Arc;

pub(crate) type AsyncCallback<InputType> = Arc<dyn Fn(InputType) -> BoxFuture<'static, ()> + Send + Sync>;

pub fn create_consumer_callback<T, F, Fut>(handler: F) -> AsyncCallback<T>
    where
        T: Send + Sync + 'static,
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    { 
        Arc::new(move |item: T| Box::pin(handler(item)))
    }