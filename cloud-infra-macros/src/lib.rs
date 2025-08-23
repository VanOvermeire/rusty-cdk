use proc_macro::TokenStream;
use quote::quote;
use std::path::{absolute, Path};
use syn::{LitInt, LitStr};

/// Generates a `StringWithOnlyAlphaNumericsAndUnderscores` after checking at compile time that the input is valid
#[proc_macro]
pub fn string_with_only_alpha_numerics_and_underscores(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if value.is_empty() {
        panic!("value should not be blank")
    }
    
    if value.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        panic!("value should only contain alphanumeric characters and underscores")
    }
    
    quote!(
        StringWithOnlyAlphaNumericsAndUnderscores(#value.to_string())
    ).into()
}

#[proc_macro]
pub fn env_var_key(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if value.len() < 2 {
        panic!("env var key should be at least two characters long")
    }

    if value.get(0..1).expect("just checked that length is at least 2") == "_" {
        panic!("env var key not start with an underscore")
    }

    if value.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        panic!("env var key should only contain alphanumeric characters and underscores")
    }

    quote!(
        EnvVarKey(#value.to_string())
    ).into()
}

#[proc_macro]
pub fn zipfile(input: TokenStream) -> TokenStream {
    let output: LitStr = syn::parse(input).unwrap();
    let value = output.value();

    if !value.ends_with(".zip") {
        panic!("zip file should end with `.zip`, instead found {}", value)
    }

    let path = Path::new(&value);

    if !path.exists() {
        panic!("did not find file {}", value)
    }

    let value = if path.is_relative() {
        let absolute_path = absolute(path).expect("to convert zip file path to an absolute path");
        absolute_path.to_str().expect("zip file path to be valid unicode").to_string()
    } else {
        path.to_str().expect("zip file path to be valid unicode").to_string()
    };

    quote!(
        ZipFile(#value.to_string())
    ).into()
}

#[proc_macro]
pub fn non_zero_number(input: TokenStream) -> TokenStream {
    let output: LitInt = syn::parse(input).unwrap();

    let as_number: syn::Result<u32> = output.base10_parse();

    let num = if let Ok(num) = as_number {
        if num == 0 {
            panic!("value should not be null")
        }
        num
    } else {
        panic!("value is not a valid number")
    };

    quote!(
        NonZeroNumber(#num)
    ).into()
}

macro_rules! number_check {
    ($name:ident,$min:literal,$max:literal,$output:ident) => {
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            let output: LitInt = syn::parse(input).unwrap();
        
            let as_number: syn::Result<u16> = output.base10_parse();
        
            let num = if let Ok(num) = as_number {
                if num < $min {
                    panic!("value should be at least {}", $min)
                } else if num > $max {
                    panic!("value should be at most {}", $max)
                }
                num
            } else {
                panic!("value is not a valid number")
            };
        
            quote!(
                $output(#num)
            ).into()
        }
    };
}

number_check!(memory, 128, 10240, Memory);
number_check!(timeout, 1, 900, Timeout);
