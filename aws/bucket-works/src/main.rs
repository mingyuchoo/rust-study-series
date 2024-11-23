use anyhow::Result;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::Client;
use std::env;
use std::error::Error;
use std::path::Path;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 4 {
        eprintln!("Usage {} <region> <bucket_name> <file_path>",
                  arguments[0].clone());
        std::process::exit(1);
    }
    let region = arguments[1].clone();
    let bucket_name = arguments[2].clone();
    let file_path = Path::new(&arguments[3]);

    let region_provider = RegionProviderChain::default_provider().or_else(aws_sdk_s3::config::Region::new(region));

    let config = aws_config::defaults(aws_config::BehaviorVersion::latest()).region(region_provider)
                                                                            .load()
                                                                            .await;

    let client = Client::new(&config);

    create_bucket(&client, &arguments[1], &bucket_name).await?;
    upload_file(&client, &bucket_name, file_path).await?;

    println!("Completed.");
    Ok(())
}

async fn create_bucket(client: &Client,
                       region: &String,
                       bucket_name: &str)
                       -> Result<(), Box<dyn Error>> {
    println!("Creating S3 bucket: {}", bucket_name);

    let location = BucketLocationConstraint::from_str(region).map_err(|_| {
                       format!("Invalid location constraint: {}", region)
                   })?;

    let config = create_bucket_config(location);

    client.create_bucket()
          .bucket(bucket_name)
          .create_bucket_configuration(config)
          .send()
          .await?;

    Ok(())
}

async fn upload_file(client: &Client,
                     bucket_name: &str,
                     file_path: &Path)
                     -> Result<(), Box<dyn Error>> {
    let file_name =
        file_path.file_name()
                 .and_then(|name| name.to_str())
                 .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;

    println!("Uploading file: {}", file_name);

    let body = ByteStream::from_path(file_path).await?;
    client.put_object()
          .bucket(bucket_name)
          .key(file_name)
          .body(body)
          .send()
          .await?;

    Ok(())
}

fn create_bucket_config(location: BucketLocationConstraint)
                        -> CreateBucketConfiguration {
    CreateBucketConfiguration::builder().location_constraint(location)
                                        .build()
}
