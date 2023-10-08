use aws_sdk_s3::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::from_env().load().await;

    let s3_config_builder =
        aws_sdk_s3::config::Builder::from(&config).endpoint_url("http://localhost:4566");
    let s3 = Client::from_conf(s3_config_builder.build());

    // List the first page of buckets in the account
    let response = s3.list_buckets().send().await?;

    // Check if the response returned any buckets
    if let Some(buckets) = response.buckets() {
        // Print each bucket name out
        for bucket in buckets {
            println!("bucket name: {}", bucket.name().unwrap());
        }
    } else {
        println!("You don't have any buckets!");
    }

    Ok(())
}
