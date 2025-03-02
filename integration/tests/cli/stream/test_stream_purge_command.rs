use crate::cli::common::{
    IggyCmdCommand, IggyCmdTest, IggyCmdTestCase, TestHelpCmd, TestStreamId, CLAP_INDENT,
    USAGE_PREFIX,
};
use assert_cmd::assert::Assert;
use async_trait::async_trait;
use iggy::client::Client;
use iggy::identifier::Identifier;
use iggy::messages::send_messages::{Message, Partitioning, SendMessages};
use iggy::streams::create_stream::CreateStream;
use iggy::streams::delete_stream::DeleteStream;
use iggy::streams::get_stream::GetStream;
use iggy::topics::create_topic::CreateTopic;
use predicates::str::diff;
use serial_test::parallel;
use std::str::FromStr;

struct TestStreamPurgeCmd {
    stream_id: u32,
    stream_name: String,
    using_identifier: TestStreamId,
    topic_id: u32,
    topic_name: String,
}

impl TestStreamPurgeCmd {
    fn new(stream_id: u32, name: String, using_identifier: TestStreamId) -> Self {
        Self {
            stream_id,
            stream_name: name,
            using_identifier,
            topic_id: 1,
            topic_name: String::from("test_topic"),
        }
    }

    fn to_arg(&self) -> String {
        match self.using_identifier {
            TestStreamId::Named => self.stream_name.clone(),
            TestStreamId::Numeric => format!("{}", self.stream_id),
        }
    }
}

#[async_trait]
impl IggyCmdTestCase for TestStreamPurgeCmd {
    async fn prepare_server_state(&mut self, client: &dyn Client) {
        let stream = client
            .create_stream(&CreateStream {
                stream_id: Some(self.stream_id),
                name: self.stream_name.clone(),
            })
            .await;
        assert!(stream.is_ok());

        let topic = client
            .create_topic(&CreateTopic {
                stream_id: Identifier::numeric(self.stream_id).unwrap(),
                topic_id: Some(self.topic_id),
                partitions_count: 10,
                message_expiry: None,
                max_topic_size: None,
                replication_factor: 1,
                name: self.topic_name.clone(),
            })
            .await;
        assert!(topic.is_ok());

        let messages = (1..100)
            .map(|n| format!("message {}", n))
            .filter_map(|s| Message::from_str(s.as_str()).ok())
            .collect::<Vec<_>>();

        let send_status = client
            .send_messages(&mut SendMessages {
                stream_id: Identifier::numeric(self.stream_id).unwrap(),
                topic_id: Identifier::numeric(self.topic_id).unwrap(),
                partitioning: Partitioning::default(),
                messages,
            })
            .await;
        assert!(send_status.is_ok());

        let stream_state = client
            .get_stream(&GetStream {
                stream_id: Identifier::numeric(self.stream_id).unwrap(),
            })
            .await;
        assert!(stream_state.is_ok());
        let stream_state = stream_state.unwrap();
        assert!(stream_state.size_bytes > 0);
    }

    fn get_command(&self) -> IggyCmdCommand {
        IggyCmdCommand::new()
            .arg("stream")
            .arg("purge")
            .arg(self.to_arg())
            .with_env_credentials()
    }

    fn verify_command(&self, command_state: Assert) {
        let stream_id = match self.using_identifier {
            TestStreamId::Named => self.stream_name.clone(),
            TestStreamId::Numeric => format!("{}", self.stream_id),
        };

        let start_message = format!(
            "Executing purge stream with ID: {}\nStream with ID: {} purged\n",
            stream_id, stream_id
        );

        command_state.success().stdout(diff(start_message));
    }

    async fn verify_server_state(&self, client: &dyn Client) {
        let stream_state = client
            .get_stream(&GetStream {
                stream_id: Identifier::numeric(self.stream_id).unwrap(),
            })
            .await;
        assert!(stream_state.is_ok());
        let stream_state = stream_state.unwrap();
        assert_eq!(stream_state.size_bytes, 0);

        let stream_delete = client
            .delete_stream(&DeleteStream {
                stream_id: Identifier::numeric(self.stream_id).unwrap(),
            })
            .await;
        assert!(stream_delete.is_ok());
    }
}

#[tokio::test]
#[parallel]
pub async fn should_be_successful() {
    let mut iggy_cmd_test = IggyCmdTest::default();

    iggy_cmd_test.setup().await;
    iggy_cmd_test
        .execute_test(TestStreamPurgeCmd::new(
            1,
            String::from("production"),
            TestStreamId::Named,
        ))
        .await;
    iggy_cmd_test
        .execute_test(TestStreamPurgeCmd::new(
            2,
            String::from("testing"),
            TestStreamId::Numeric,
        ))
        .await;
}

#[tokio::test]
#[parallel]
pub async fn should_help_match() {
    let mut iggy_cmd_test = IggyCmdTest::default();

    iggy_cmd_test
        .execute_test_for_help_command(TestHelpCmd::new(
            vec!["stream", "purge", "--help"],
            format!(
                r#"Purge all topics in given stream ID

Command removes all messages from given stream
Stream ID can be specified as a stream name or ID

Examples:
 iggy stream purge 1
 iggy stream purge test

{USAGE_PREFIX} stream purge <STREAM_ID>

Arguments:
  <STREAM_ID>
          Stream ID to purge
{CLAP_INDENT}
          Stream ID can be specified as a stream name or ID

Options:
  -h, --help
          Print help (see a summary with '-h')
"#,
            ),
        ))
        .await;
}

#[tokio::test]
#[parallel]
pub async fn should_short_help_match() {
    let mut iggy_cmd_test = IggyCmdTest::default();

    iggy_cmd_test
        .execute_test_for_help_command(TestHelpCmd::new(
            vec!["stream", "purge", "-h"],
            format!(
                r#"Purge all topics in given stream ID

{USAGE_PREFIX} stream purge <STREAM_ID>

Arguments:
  <STREAM_ID>  Stream ID to purge

Options:
  -h, --help  Print help (see more with '--help')
"#,
            ),
        ))
        .await;
}
