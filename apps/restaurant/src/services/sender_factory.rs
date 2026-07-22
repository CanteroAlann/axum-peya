use crate::services::sender::Sender;
use async_trait::async_trait;


#[async_trait]
pub trait SenderFactory {
    async fn create_sender() -> Sender;
}



