use crate::client::MessageClient;
use crate::error::IggyError;
use crate::http::client::HttpClient;
use crate::messages::poll_messages::PollMessages;
use crate::messages::send_messages::SendMessages;
use crate::models::messages::PolledMessages;
use async_trait::async_trait;

#[async_trait]
impl MessageClient for HttpClient {
    async fn poll_messages(&self, command: &PollMessages) -> Result<PolledMessages, IggyError> {
        let response = self
            .get_with_query(
                &get_path(
                    &command.stream_id.as_string(),
                    &command.topic_id.as_string(),
                ),
                &command,
            )
            .await?;
        let messages = response.json().await?;
        Ok(messages)
    }

    async fn send_messages(&self, command: &mut SendMessages) -> Result<(), IggyError> {
        self.post(
            &get_path(
                &command.stream_id.as_string(),
                &command.topic_id.as_string(),
            ),
            &command,
        )
        .await?;
        Ok(())
    }
}

fn get_path(stream_id: &str, topic_id: &str) -> String {
    format!("streams/{stream_id}/topics/{topic_id}/messages")
}
