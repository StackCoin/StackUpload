use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::S3Error;
use uuid::Uuid;

struct Storage {
    region: Region,
    credentials: Credentials,
    bucket: String,
}

pub fn put_presigned_url_with_uuid(bucket: &Bucket, expire_secs: u32) -> Result<String, S3Error> {
    bucket.presign_put(Uuid::new_v4().to_simple().to_string(), expire_secs)
}

pub fn connect_bucket() -> Result<Bucket, S3Error> {
    let dotenv_stackupload_region = dotenv::var("STACKUPLOAD_REGION").unwrap();
    let credentials = Credentials::from_env_specific(
        Some("S3_ACCESS_KEY_ID"),
        Some("S3_SECRET_ACCESS_KEY"),
        None,
        None,
    )?;
    let backend = Storage {
        region: dotenv_stackupload_region.parse()?,
        credentials,
        bucket: "stackmarket".to_string(),
    };

    Ok(Bucket::new(
        &backend.bucket,
        backend.region,
        backend.credentials,
    )?)
}
