use crate::configs::resource_quota::MemoryResourceQuota;
use iggy::utils::byte_size::IggyByteSize;
use iggy::{
    compression::compression_algorithm::CompressionAlgorithm, utils::duration::IggyDuration,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemConfig {
    pub path: String,
    pub database: DatabaseConfig,
    pub runtime: RuntimeConfig,
    pub logging: LoggingConfig,
    pub cache: CacheConfig,
    pub retention_policy: RetentionPolicyConfig,
    pub stream: StreamConfig,
    pub topic: TopicConfig,
    pub partition: PartitionConfig,
    pub segment: SegmentConfig,
    pub encryption: EncryptionConfig,
    pub compression: CompressionConfig,
    pub message_deduplication: MessageDeduplicationConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RuntimeConfig {
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompressionConfig {
    pub allow_override: bool,
    pub default_algorithm: CompressionAlgorithm,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub path: String,
    pub level: String,
    pub max_size: IggyByteSize,
    #[serde_as(as = "DisplayFromStr")]
    pub retention: IggyDuration,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub size: MemoryResourceQuota,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct RetentionPolicyConfig {
    #[serde_as(as = "DisplayFromStr")]
    pub message_expiry: IggyDuration,
    pub max_topic_size: IggyByteSize,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct EncryptionConfig {
    pub enabled: bool,
    pub key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamConfig {
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TopicConfig {
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartitionConfig {
    pub path: String,
    pub messages_required_to_save: u32,
    pub enforce_fsync: bool,
    pub validate_checksum: bool,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageDeduplicationConfig {
    pub enabled: bool,
    pub max_entries: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub expiry: IggyDuration,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SegmentConfig {
    pub size: IggyByteSize,
    pub cache_indexes: bool,
    pub cache_time_indexes: bool,
}

impl SystemConfig {
    pub fn get_system_path(&self) -> String {
        self.path.to_string()
    }

    pub fn get_database_path(&self) -> String {
        format!("{}/{}", self.get_system_path(), self.database.path)
    }

    pub fn get_runtime_path(&self) -> String {
        format!("{}/{}", self.get_system_path(), self.runtime.path)
    }

    pub fn get_streams_path(&self) -> String {
        format!("{}/{}", self.get_system_path(), self.stream.path)
    }

    pub fn get_stream_path(&self, stream_id: u32) -> String {
        format!("{}/{}", self.get_streams_path(), stream_id)
    }

    pub fn get_topics_path(&self, stream_id: u32) -> String {
        format!("{}/{}", self.get_stream_path(stream_id), self.topic.path)
    }

    pub fn get_topic_path(&self, stream_id: u32, topic_id: u32) -> String {
        format!("{}/{}", self.get_topics_path(stream_id), topic_id)
    }

    pub fn get_partitions_path(&self, stream_id: u32, topic_id: u32) -> String {
        format!(
            "{}/{}",
            self.get_topic_path(stream_id, topic_id),
            self.partition.path
        )
    }

    pub fn get_partition_path(&self, stream_id: u32, topic_id: u32, partition_id: u32) -> String {
        format!(
            "{}/{}",
            self.get_partitions_path(stream_id, topic_id),
            partition_id
        )
    }

    pub fn get_segment_path(
        &self,
        stream_id: u32,
        topic_id: u32,
        partition_id: u32,
        start_offset: u64,
    ) -> String {
        format!(
            "{}/{:0>20}",
            self.get_partition_path(stream_id, topic_id, partition_id),
            start_offset
        )
    }
}
