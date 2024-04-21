use std::time::Duration;
use rdkafka::util::AsyncRuntime;

pub struct TokioRuntime;

impl AsyncRuntime for TokioRuntime {
    type Delay = tokio::time::Sleep;
    
    fn spawn<T>(task: T)
        where
            T: std::future::Future<Output = ()> + Send + 'static,
    {
        tokio::spawn(task);
    }
    
    fn delay_for(duration: Duration) -> Self::Delay {
        tokio::time::sleep(duration)
    }
}
