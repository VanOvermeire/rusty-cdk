pub mod apigateway;
pub mod appconfig;
pub mod appsync;
pub mod cloudfront;
pub mod cloudwatch;
pub mod dynamodb;
pub mod events;
pub mod iam;
pub mod kms;
pub mod lambda;
pub mod s3;
pub mod secretsmanager;
pub mod shared;
pub mod sns;
pub mod sqs;
pub mod stack;
pub mod wrappers;

// keep this one private for now, if made public, changes should be made to contract of resources (see the module for details)
mod custom_resource; 
mod intrinsic;



