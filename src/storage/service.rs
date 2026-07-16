use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_credential_types::Credentials;

use crate::config::Config;

pub struct StorageService {
    pub client: Client,
}

impl StorageService {
    pub async fn new(cfg: &Config) -> Result<Self> {
        let credentials = Credentials::new(
            cfg.b2_key_id.clone(),
            cfg.b2_application_key.clone(),
            None,
            None,
            "backblaze-b2",
        );

        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(credentials)
            .region(aws_sdk_s3::config::Region::new("us-west-004"))
            .load()
            .await;

        let client = Client::new(&shared_config);

        Ok(Self { client })
    }
}
