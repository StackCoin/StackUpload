use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::S3Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct PresignedURLResponse {
    #[serde(rename(serialize = "specialName"))]
    special_name: String,
    #[serde(rename(serialize = "uploadURL"))]
    upload_url: String,
}

pub fn put_presigned_url_with_uuid(bucket: &Bucket, expire_secs: u32) -> Result<String, S3Error> {
    let name = Uuid::new_v4().to_simple().to_string();
    Ok(serde_json::to_string(&PresignedURLResponse {
        special_name: name.to_owned(),
        upload_url: bucket.presign_put(name.to_owned(), expire_secs)?,
    })
    .expect("Incorrect response input"))
}

pub fn get_dotenv_var_expect(name: &str) -> String {
    dotenv::var(name).expect(&format!("{} not found in environment variables. If this is intentional, simpy set the value to an empty value", name).to_owned())
}

pub fn connect_bucket() -> Result<Bucket, S3Error> {
    let dotenv_stackupload_region = get_dotenv_var_expect("STACKUPLOAD_REGION");
    let dotenv_stackupload_endpoint = get_dotenv_var_expect("STACKUPLOAD_ENDPOINT");
    let dotenv_stackupload_bucket = get_dotenv_var_expect("STACKUPLOAD_BUCKET");
    let dotenv_stackupload_url_style =
        dotenv::var("STACKUPLOAD_URL_STYLE").unwrap_or("path".to_owned());

    let region = Region::Custom {
        region: dotenv_stackupload_region,
        endpoint: dotenv_stackupload_endpoint,
    };
    let credentials =
        Credentials::from_env_specific(Some("S3_ACCESS_KEY"), Some("S3_SECRET_KEY"), None, None)?;

    if dotenv_stackupload_url_style == "path" {
        Bucket::new_with_path_style(&dotenv_stackupload_bucket, region, credentials)
    } else {
        Bucket::new(&dotenv_stackupload_bucket, region, credentials)
    }
}
