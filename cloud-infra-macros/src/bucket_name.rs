use proc_macro::TokenStream;
use std::collections::HashSet;
use std::fs::read_to_string;
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::{Error, LitStr};
use crate::bucket_name::FileStorageOutput::{Invalid, Unknown, Valid};
use crate::file_util;

const BUCKET_NAMES_INFO_FILE: &str = ".cloud_infra_bucket_name_info";

// all of this is very similar to the logic in bucket.rs...
// ideally, you'd keep a date when you checked this and recheck if a significant amount of time (a few months?) has passed
#[derive(Deserialize, Serialize)]
struct BucketNameInfo<'a> {
    #[serde(borrow)]
    valid_bucket_names: HashSet<&'a str>,
    invalid_bucket_names: HashSet<&'a str>
}

impl BucketNameInfo<'_> {
    fn new() -> Self {
        Self { valid_bucket_names: Default::default(), invalid_bucket_names: Default::default() }
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

pub(crate) fn valid_bucket_name_according_to_file_storage(value: &str) -> FileStorageOutput {
    let full_path = match file_util::get_file_path(BUCKET_NAMES_INFO_FILE) {
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
        match serde_json::from_str::<BucketNameInfo>(&file_as_string) {
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
    let info_as_string = file_util::read_info(BUCKET_NAMES_INFO_FILE).unwrap_or("{}".to_string());

    match serde_json::from_str::<BucketNameInfo>(&info_as_string) {
        Ok(mut info) => {
            match input {
                FileStorageInput::Valid(name) => {
                    info.valid_bucket_names.insert(&name);
                    info.invalid_bucket_names = info.invalid_bucket_names.into_iter().filter(|v| *v != name).collect()
                },
                FileStorageInput::Invalid(name) => {
                    info.invalid_bucket_names.insert(&name);
                    info.valid_bucket_names = info.valid_bucket_names.into_iter().filter(|v| *v != name).collect()
                },
            };
            file_util::write_info(BUCKET_NAMES_INFO_FILE, info);
        }
        Err(_) => write_empty_bucket_info(),
    }
}

fn write_empty_bucket_info() {
    file_util::write_info(BUCKET_NAMES_INFO_FILE, BucketNameInfo::new());
}

pub(crate) fn check_bucket_name(input: LitStr) -> Result<(), Error> {
    let name = input.value();

    let url = format!("https://{}.s3.amazonaws.com/", name);
    let response = reqwest::blocking::get(&url);
    
    if let Ok(response) = response {
        if response.status() == 404 {
            Ok(())
        } else if response.status() == 403 {
            Err(Error::new(input.span(), "bucket name is already taken"))
        } else {
            Err(Error::new(input.span(), "could not check bucket name"))
        }
    } else {
        Err(Error::new(input.span(), "could not check bucket name"))
    }
}

pub(crate) fn bucket_name_output(value: String) -> TokenStream {
    quote!(
        BucketName(#value.to_string())
    ).into()
}