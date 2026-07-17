impl StorageService {
    pub async fn new(cfg: &Config) -> Result<Self> {
        let credentials = Credentials::new(
            cfg.b2_key_id.clone(),
            cfg.b2_application_key.clone(),
            None,
            None,
            "backblaze-b2",
        );

        let s3_config = Builder::new()
            .credentials_provider(credentials)
            .region(Region::new("us-east-005"))
            .endpoint_url("https://s3.us-east-005.backblazeb2.com")
            .force_path_style(true)
            .behavior_version(BehaviorVersion::latest())
            .build();

        let client = Client::from_conf(s3_config);

        Ok(Self {
            client,
            bucket: cfg.b2_bucket_name.clone(),
        })
    }

    pub async fn upload(
        &self,
        key: String,
        data: Bytes,
        content_type: String,
    ) -> Result<String> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(data.into())
            .content_type(content_type)
            .send()
            .await?;

        Ok(key)
    }

    pub async fn delete(
        &self,
        key: String,
    ) -> Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }
}
