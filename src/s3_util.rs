use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::S3Error;
use uuid::Uuid;

pub fn put_presigned_url_with_uuid(bucket: &Bucket, expire_secs: u32) -> Result<String, S3Error> {
    bucket.presign_put(Uuid::new_v4().to_simple().to_string(), expire_secs)
}

pub fn connect_bucket() -> Result<Bucket, S3Error> {
    let dotenv_stackupload_region = dotenv::var("STACKUPLOAD_REGION").unwrap();
    let dotenv_stackupload_endpoint = dotenv::var("STACKUPLOAD_ENDPOINT").unwrap();
    let dotenv_stackupload_bucket = dotenv::var("STACKUPLOAD_BUCKET").unwrap();
    let dotenv_stackupload_url_style =
        dotenv::var("STACKUPLOAD_URL_STYLE").unwrap_or("path".to_owned());
    let region = Region::Custom {
        region: dotenv_stackupload_region,
        endpoint: dotenv_stackupload_endpoint,
    };
    let credentials = Credentials::from_env_specific(
        Some("S3_ACCESS_KEY_ID"),
        Some("S3_SECRET_ACCESS_KEY"),
        None,
        None,
    )?;

    if dotenv_stackupload_url_style == "path" {
        Bucket::new(&dotenv_stackupload_bucket, region, credentials)
    } else {
        Bucket::new_with_path_style(&dotenv_stackupload_bucket, region, credentials)
    }
}
