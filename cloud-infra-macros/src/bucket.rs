use proc_macro::TokenStream;
use std::collections::HashSet;
use std::fs::{read_to_string, write};
use std::path::{PathBuf};
use dirs::home_dir;
use quote::quote;
use syn::{Error, LitStr};
use serde::{Deserialize, Serialize};
use crate::bucket::FileStorageOutput::{Invalid, Unknown, Valid};

const HOME_DIR_CLOUD_BUCKET_FILE: &str = ".cloud_infra_bucket_info";

#[derive(Deserialize, Serialize)]
struct BucketInfo<'a> {
    #[serde(borrow)]
    valid_bucket_names: HashSet<&'a str>,
    invalid_bucket_names: HashSet<&'a str>
}

impl BucketInfo<'_> {
    fn new() -> Self {
        Self { valid_bucket_names: HashSet::new(), invalid_bucket_names: HashSet::new() }
    }
}

pub(crate) enum FileStorageInput<'a> {
    Valid(&'a str),
    Invalid(&'a str),
}

pub(crate) enum FileStorageOutput {
    Valid,
    Invalid,
    Unknown,
}

pub(crate) fn valid_bucket_according_to_file_storage(value: &str) -> FileStorageOutput {
    let full_path = match get_file_path() {
        Some(p) => p,
        None => {
            return Unknown
        }
    };

    if full_path.exists() {
        let file_as_string = match read_to_string(full_path) {
            Ok(str) => str,
            Err(_) => {
                write_empty_bucket_info();
                return Unknown
            }
        };
        match serde_json::from_str::<BucketInfo>(&file_as_string) {
            Ok(info) => {
                let valid_bucket_name = info.valid_bucket_names.iter().find(|v| **v == value);
                let invalid_bucket_name = info.invalid_bucket_names.iter().find(|v| **v == value);

                if valid_bucket_name.is_some() {
                    return Valid
                } else if invalid_bucket_name.is_some() {
                    return Invalid
                }
            }
            Err(_) => {
                write_empty_bucket_info();
                return Unknown
            }
        }
    }
    Unknown
}

pub(crate) fn update_file_storage(input: FileStorageInput) {
    let info_as_string = read_bucket_info().unwrap_or("{}".to_string());

    match serde_json::from_str::<BucketInfo>(&info_as_string) {
        Ok(mut info) => {
            match input {
                FileStorageInput::Valid(name) => info.valid_bucket_names.insert(&name),
                FileStorageInput::Invalid(name) => info.invalid_bucket_names.insert(&name),
            };
            write_bucket_info(info);
        }
        Err(_) => write_empty_bucket_info(),
    }
}

fn read_bucket_info() -> Option<String> {
    get_file_path().and_then(|p| read_to_string(p).ok())
}

fn write_empty_bucket_info() {
    write_bucket_info(BucketInfo::new());
}

fn write_bucket_info(info: BucketInfo) {
    match get_file_path() {
        Some(path) => {
            let info_as_string = serde_json::to_string(&info).expect("to be able to serialize bucket info");
            let _result = write(&path, info_as_string);
        }
        None => {}
    }
}

fn get_file_path() -> Option<PathBuf> {
    home_dir().map(|home_dir| {
        home_dir.join(HOME_DIR_CLOUD_BUCKET_FILE)
    })
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