mod s3_util;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use s3::bucket::Bucket;
use s3::S3Error;
use s3_util::{get_presigned_url_with_uuid, put_presigned_url_with_uuid};

const PRESIGN_EXPIRE_SECS: u32 = 300;

struct AppState {
    bucket: Bucket,
}

trait ToHttp {
    fn to_http(self: Self) -> HttpResponse;
}

impl ToHttp for String {
    fn to_http(self: String) -> HttpResponse {
        HttpResponse::Ok().body(self)
    }
}

impl ToHttp for S3Error {
    fn to_http(self: S3Error) -> HttpResponse {
        HttpResponse::InternalServerError().body(self.data.unwrap_or("No Subject".to_string()))
    }
}

#[get("url/put")]
async fn put_presigned_url(data: web::Data<AppState>) -> HttpResponse {
    match put_presigned_url_with_uuid(&data.bucket, PRESIGN_EXPIRE_SECS) {
        Ok(url) => url.to_http(),
        Err(error) => error.to_http(),
    }
}

#[get("url/get/{name}")]
async fn get_presigned_url(
    data: web::Data<AppState>,
    web::Path(name): web::Path<String>,
) -> HttpResponse {
    match get_presigned_url_with_uuid(&data.bucket, PRESIGN_EXPIRE_SECS, name) {
        Ok(url) => url.to_http(),
        Err(error) => error.to_http(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let dotenv_stackupload_ip_string = s3_util::get_dotenv_var_expect("STACKUPLOAD_IP_STRING");
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                bucket: s3_util::connect_bucket().unwrap(),
            })
            .service(put_presigned_url)
            .service(get_presigned_url)
    })
    .bind(dotenv_stackupload_ip_string)?
    .run()
    .await
}
