use crate::helpers;
use s3::bucket::Bucket;
use s3::creds::Credentials;

pub fn get_s3_bucket() -> Bucket {
  let s3_bucket_name = helpers::get_env("S3_BUCKET_NAME");
  let s3_bucket_region: s3::Region = helpers::get_env("S3_BUCKET_REGION").parse().unwrap();
  let aws_bucket_access_key = helpers::get_env("AWS_BUCKET_ACCESS_KEY");
  let aws_bucket_secret_key = helpers::get_env("AWS_BUCKET_SECRET_KEY");
  let credentials = Credentials::new(
    Some(aws_bucket_access_key.as_str()),
    Some(aws_bucket_secret_key.as_str()),
    None,
    None,
    None,
  )
  .unwrap();
  let bucket = Bucket::new(&s3_bucket_name, s3_bucket_region, credentials).unwrap();

  match bucket.presign_get(
    "/images/1/2023-05-08 23:04:48.222-20230508_170806.jpg",
    100,
    None,
  ) {
    Ok(url) => println!("presigned url: {}", url),
    Err(e) => {
      println!("Error generating presigned url: {}", e);
    }
  }

  bucket
}
