use std::str;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::S3Error;
use uuid::Uuid;

struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

const MESSAGE: &str = "I want to go to S3";

pub fn put_presigned_url_with_uuid(bucket: &Bucket, expire_secs: u32) -> Result<String, S3Error> {
    bucket.presign_put(Uuid::new_v4().to_simple().to_string(), expire_secs)
}

