use std::ops::Add;
use anyhow::bail;
use aws_config::BehaviorVersion;
use aws_sdk_s3 as s3;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
    let client = aws_sdk_s3::Client::new(&config);

    // List addressable buckets
    println!("Buckets:");
    let buckets = match client.list_buckets().send().await {
        Ok(buckets) => buckets,
        Err(e) => bail!("{}", e),
    };
    for bucket in buckets.buckets() {
        let Some(ref name) = bucket.name else {
            continue;
        };
        println!("{}", name)
    }

    // List all objects within the Klothed production bucket.
    let klothed_bucket = "".to_string();
    let list_objects_output = match client.list_objects_v2().bucket(klothed_bucket).send().await {
        Ok(list_objects_output) => list_objects_output,
        Err(e) => bail!("{}", e),
    };

    // Enumerate over all objects in the bucket
    println!("Objects:");
    for (count, obj) in list_objects_output.contents().iter().enumerate() {
        if count >= 1000 {
            break
        }
        let key_name = obj.key?;
        println!("Object Key: {}", key_name);
    }

    Ok(())
}
