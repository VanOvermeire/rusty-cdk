use proc_macro::TokenStream;
use std::collections::HashSet;
use std::fs::{read_to_string};
use quote::quote;
use syn::{Error, LitStr};
use serde::{Deserialize, Serialize};
use crate::bucket::FileStorageOutput::{Invalid, Unknown, Valid};
use crate::file_util;

const BUCKET_INFO_FILE: &str = ".cloud_infra_bucket_info";

#[derive(Deserialize, Serialize)]
struct BucketInfo<'a> {
    #[serde(borrow)]
    real_bucket_names: HashSet<&'a str>,
    unknown_bucket_names: HashSet<&'a str>
}

impl BucketInfo<'_> {
    fn new() -> Self {
        Self { real_bucket_names: HashSet::new(), unknown_bucket_names: HashSet::new() }
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
    let full_path = match file_util::get_file_path(BUCKET_INFO_FILE) {
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
                let valid_bucket_name = info.real_bucket_names.iter().find(|v| **v == value);
                let invalid_bucket_name = info.unknown_bucket_names.iter().find(|v| **v == value);

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
    let info_as_string = file_util::read_info(BUCKET_INFO_FILE).unwrap_or("{}".to_string());

    match serde_json::from_str::<BucketInfo>(&info_as_string) {
        Ok(mut info) => {
            match input {
                FileStorageInput::Valid(name) => {
                    info.real_bucket_names.insert(&name);
                    info.unknown_bucket_names = info.unknown_bucket_names.into_iter().filter(|v| *v != name).collect()
                },
                FileStorageInput::Invalid(name) => {
                    info.unknown_bucket_names.insert(&name);
                    info.real_bucket_names = info.real_bucket_names.into_iter().filter(|v| *v != name).collect()
                },
            };
            file_util::write_info(BUCKET_INFO_FILE, info);
        }
        Err(_) => write_empty_bucket_info(),
    }
}

fn write_empty_bucket_info() {
    file_util::write_info(BUCKET_INFO_FILE, BucketInfo::new());
}

pub(crate) async fn find_bucket(input: LitStr) -> Result<(), Error> {
    let name = input.value();
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
                    return Err(Error::new(input.span(), format!("did not find bucket with name {} in your account", name)))
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