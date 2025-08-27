use proc_macro::TokenStream;
use std::fs::{create_dir, read_to_string, write};
use std::path::Path;
use quote::quote;
use syn::{Error, LitStr};
use serde::{Deserialize, Serialize};
use crate::bucket::FileStorageOutput::{INVALID, UNKNOWN, VALID};

const HOME_DIR_CLOUD_INFRA: &str = "~/.cloud_infra";
const BUCKETS_FILE: &str = "buckets";

// TODO borrow more

#[derive(Deserialize, Serialize)]
struct BucketInfo {
    valid_bucket_names: Vec<String>,
    invalid_bucket_names: Vec<String>
}

impl BucketInfo {
    fn new() -> Self {
        Self { valid_bucket_names: vec![], invalid_bucket_names: vec![] }
    }
}

pub(crate) enum FileStorageInput {
    VALID(String),
    INVALID(String),
}

pub(crate) enum FileStorageOutput {
    VALID,
    INVALID,
    UNKNOWN,
}

pub(crate) fn valid_bucket_according_to_file_storage(value: &str) -> FileStorageOutput {
    let path_as_string = format!("{}/{}", HOME_DIR_CLOUD_INFRA, BUCKETS_FILE);
    let path = Path::new(&path_as_string);

    if path.exists() {
        let as_string = match read_to_string(path) {
            Ok(str) => str,
            Err(_) => {
                write_empty_bucket_info();
                return UNKNOWN
            }
        };
        match serde_json::from_str::<BucketInfo>(&as_string) {
            Ok(info) => {
                let valid_bucket_name = info.valid_bucket_names.iter().find(|v| v.as_str() == value);

                if let Some(_) = valid_bucket_name {
                    return VALID
                }

                let invalid_bucket_name = info.valid_bucket_names.iter().find(|v| v.as_str() == value);

                if let Some(_) = invalid_bucket_name {
                    return INVALID
                }
            }
            Err(_) => write_empty_bucket_info(),
        }
    }
    UNKNOWN
}

pub(crate) fn update_file_storage(input: FileStorageInput) {
    let _result = create_dir(HOME_DIR_CLOUD_INFRA);
    
    let path_as_string = format!("{}/{}", HOME_DIR_CLOUD_INFRA, BUCKETS_FILE);
    let path = Path::new(&path_as_string);

    let as_string = read_to_string(path).expect("our own file to be readable as string");

    match serde_json::from_str::<BucketInfo>(&as_string) {
        Ok(mut info) => {
            match input {
                FileStorageInput::VALID(name) => info.valid_bucket_names.push(name),
                FileStorageInput::INVALID(name) => info.invalid_bucket_names.push(name),
            }
            write_bucket_info(info);
        }
        Err(_) => write_empty_bucket_info(),
    }
}

fn write_empty_bucket_info() {
    write_bucket_info(BucketInfo::new());
}

fn write_bucket_info(info: BucketInfo) {
    let path_as_string = format!("{}/{}", HOME_DIR_CLOUD_INFRA, BUCKETS_FILE);
    let path = Path::new(&path_as_string);
    
    let info_as_string = serde_json::to_string(&info).expect("to be able to serialize bucket info");
    let _result = write(&path, info_as_string);
}

pub(crate) async fn find_bucket(input: LitStr, name: &str) -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    let results = s3_client.list_buckets()
        .send()
        .await;

    match results {
        Ok(result) => {
            if let Some(buckets) = result.buckets {
                let bucket = buckets.into_iter().find(|b| b.name.iter().any(|n| n.as_str() == name));

                if bucket.is_none() {
                    return Err(Error::new(input.span(), format!("did not find bucket with name {}", name)))
                }
            } else {
                return Err(Error::new(input.span(), "no buckets found".to_string()))
            }
        }
        Err(e) => {
            return Err(Error::new(input.span(), format!("could not retrieve buckets: {e:?}")))
        }
    }

    Ok(())
}

pub(crate) fn bucket_output(value: String) -> TokenStream {
    quote!(
        Bucket(#value.to_string())
    ).into()
}