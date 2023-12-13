use std::ops::Add;
use byte_unit::{Byte, Unit};

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    // List all objects within the Klothed production bucket.
    let mut continuation_token : Option<String> = None;
    let mut total: u128 = 0;
    loop {
        let list_objects_output =  client.list_objects_v2().bucket("klthd-storage-prod")
            .set_prefix(Some("retailer".to_string()))
            .set_continuation_token(continuation_token)
            .send().await?;
        for content in list_objects_output.contents() {
            let Some(key) = content.key() else {
                continue
            };
            if !key.ends_with("/images/image.png") {
                continue
            }
            let Some(size) = content.size() else {
                continue
            };

            total = total.add(1u128);
            println!("{}", total);

        }
        // let byte = Byte::from_u128(total).unwrap_or_default();
        // // let adj_byte = byte.get_adjusted_unit(Unit::GB);
        // println!("{}", adj_byte.to_string());

        if let None = list_objects_output.next_continuation_token() {
            break
        }
        continuation_token = list_objects_output.next_continuation_token;
    }

    Ok(())
}
