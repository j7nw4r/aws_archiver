use aws_sdk_s3::Client;
use aws_sdk_s3::error::ProvideErrorMetadata;

const BUCKET_NAME: &str = "klthd-storage-prod";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    delete_individually(client).await
}

async fn delete_individually(client: Client) -> anyhow::Result<()> {
    let mut continuation_token: Option<String> = None;
    loop {
        let list_objects_output = client
            .list_objects_v2()
            .bucket(BUCKET_NAME)
            .set_prefix(Some("retailer".to_string()))
            .set_continuation_token(continuation_token)
            .send()
            .await?;
        for content in list_objects_output.contents() {
            let Some(key) = content.key() else { continue };
            if key.ends_with("/images/image.png") {
                continue;
            }

            println!("Deleting: {}", key);
            let _delete_object = match client
                .delete_object()
                .bucket(BUCKET_NAME)
                .key(key)
                .send()
                .await
            {
                Ok(delete_object) => delete_object,
                Err(e) => {
                    println!("could not delete: {}\n (message: {})", key, e.message().unwrap_or_default());
                    continue;
                }
            };
        }

        if let None = list_objects_output.next_continuation_token() {
            break;
        };

        continuation_token = list_objects_output.next_continuation_token;
    }

    Ok(())
}
