use crate::custom_resource::{BucketNotification, BucketNotificationProperties, BucketNotificationRef, LambdaFunctionConfiguration, NotificationConfiguration, TopicConfiguration};
use crate::shared::Id;
use crate::stack::{Resource, StackBuilder};
use serde_json::Value;

/// This code wqs taken from the AWS CDK
pub(crate) const BUCKET_NOTIFICATION_HANDLER_CODE: &str = r#"import boto3  # type: ignore
import json
import logging
import urllib.request

s3 = boto3.client("s3")

EVENTBRIDGE_CONFIGURATION = 'EventBridgeConfiguration'
CONFIGURATION_TYPES = ["TopicConfigurations", "QueueConfigurations", "LambdaFunctionConfigurations"]

def handler(event: dict, context):
  response_status = "SUCCESS"
  error_message = ""
  try:
    props = event["ResourceProperties"]
    notification_configuration = props["NotificationConfiguration"]
    managed = props.get('Managed', 'true').lower() == 'true'
    skipDestinationValidation = props.get('SkipDestinationValidation', 'false').lower() == 'true'
    stack_id = event['StackId']
    old = event.get("OldResourceProperties", {}).get("NotificationConfiguration", {})
    if managed:
      config = handle_managed(event["RequestType"], notification_configuration)
    else:
      config = handle_unmanaged(props["BucketName"], stack_id, event["RequestType"], notification_configuration, old)
    s3.put_bucket_notification_configuration(Bucket=props["BucketName"], NotificationConfiguration=config, SkipDestinationValidation=skipDestinationValidation)
  except Exception as e:
    logging.exception("Failed to put bucket notification configuration")
    response_status = "FAILED"
    error_message = f"Error: {str(e)}. "
  finally:
    submit_response(event, context, response_status, error_message)

def handle_managed(request_type, notification_configuration):
  if request_type == 'Delete':
    return {}
  return notification_configuration

def handle_unmanaged(bucket, stack_id, request_type, notification_configuration, old):
  def get_id(n):
    n['Id'] = ''
    sorted_notifications = sort_filter_rules(n)
    strToHash=json.dumps(sorted_notifications, sort_keys=True).replace('"Name": "prefix"', '"Name": "Prefix"').replace('"Name": "suffix"', '"Name": "Suffix"')
    return f"{stack_id}-{hash(strToHash)}"
  def with_id(n):
    n['Id'] = get_id(n)
    return n

  external_notifications = {}
  existing_notifications = s3.get_bucket_notification_configuration(Bucket=bucket)
  for t in CONFIGURATION_TYPES:
    if request_type == 'Update':
        old_incoming_ids = [get_id(n) for n in old.get(t, [])]
        external_notifications[t] = [n for n in existing_notifications.get(t, []) if not get_id(n) in old_incoming_ids]      
    elif request_type == 'Delete':
        external_notifications[t] = [n for n in existing_notifications.get(t, []) if not n['Id'].startswith(f"{stack_id}-")]
    elif request_type == 'Create':
        external_notifications[t] = [n for n in existing_notifications.get(t, [])]
  if EVENTBRIDGE_CONFIGURATION in existing_notifications:
    external_notifications[EVENTBRIDGE_CONFIGURATION] = existing_notifications[EVENTBRIDGE_CONFIGURATION]

  if request_type == 'Delete':
    return external_notifications

  notifications = {}
  for t in CONFIGURATION_TYPES:
    external = external_notifications.get(t, [])
    incoming = [with_id(n) for n in notification_configuration.get(t, [])]
    notifications[t] = external + incoming

  if EVENTBRIDGE_CONFIGURATION in notification_configuration:
    notifications[EVENTBRIDGE_CONFIGURATION] = notification_configuration[EVENTBRIDGE_CONFIGURATION]
  elif EVENTBRIDGE_CONFIGURATION in external_notifications:
    notifications[EVENTBRIDGE_CONFIGURATION] = external_notifications[EVENTBRIDGE_CONFIGURATION]

  return notifications

def submit_response(event: dict, context, response_status: str, error_message: str):
  response_body = json.dumps(
    {
      "Status": response_status,
      "Reason": f"{error_message}See the details in CloudWatch Log Stream: {context.log_stream_name}",
      "PhysicalResourceId": event.get("PhysicalResourceId") or event["LogicalResourceId"],
      "StackId": event["StackId"],
      "RequestId": event["RequestId"],
      "LogicalResourceId": event["LogicalResourceId"],
      "NoEcho": False,
    }
  ).encode("utf-8")
  headers = {"content-type": "", "content-length": str(len(response_body))}
  try:
    req = urllib.request.Request(url=event["ResponseURL"], headers=headers, data=response_body, method="PUT")
    with urllib.request.urlopen(req) as response:
      print(response.read().decode("utf-8"))
    print("Status code: " + response.reason)
  except Exception as e:
      print("send(..) failed executing request.urlopen(..): " + str(e))

def sort_filter_rules(json_obj):
  if not isinstance(json_obj, dict):
      return json_obj
  for key, value in json_obj.items():
      if isinstance(value, dict):
          json_obj[key] = sort_filter_rules(value)
      elif isinstance(value, list):
          json_obj[key] = [sort_filter_rules(item) for item in value]
  if "Filter" in json_obj and "Key" in json_obj["Filter"] and "FilterRules" in json_obj["Filter"]["Key"]:
      filter_rules = json_obj["Filter"]["Key"]["FilterRules"]
      sorted_filter_rules = sorted(filter_rules, key=lambda x: x["Name"])
      json_obj["Filter"]["Key"]["FilterRules"] = sorted_filter_rules
  return json_obj"#;

pub struct BucketNotificationBuilder {
    id: Id,
    handler_arn: Value,
    bucket_ref: Value,
    lambda_arn: Option<Value>,
    sns_ref: Option<Value>,
    dependency: Id,
}

impl BucketNotificationBuilder {
    // if this was to become public outside the crate, it should
    // - accept Refs instead of Values
    // - have different param names
    // - and only one `new`
    pub(crate) fn new_for_lambda(id: &str, handler_arn: Value, bucket_ref: Value, lambda_arn: Value, dependency: Id) -> Self {
        Self { id: Id(id.to_string()), handler_arn, bucket_ref, lambda_arn: Some(lambda_arn), sns_ref: None, dependency }
    }

    pub(crate) fn new_for_sns(id: &str, handler_arn: Value, bucket_ref: Value, sns_ref: Value, dependency: Id) -> Self {
        Self { id: Id(id.to_string()), handler_arn, bucket_ref, lambda_arn: None, sns_ref: Some(sns_ref), dependency }
    }

    pub(crate) fn build(self, stack_builder: &mut StackBuilder) -> BucketNotificationRef {
        let resource_id = Resource::generate_id("BucketNotification");
        let bucket_notification_ref = BucketNotificationRef::new(resource_id.to_string());
        
        let config = if let Some(arn) = self.lambda_arn {
            NotificationConfiguration {
                lambda_configs: Some(vec![
                    LambdaFunctionConfiguration {
                        events: vec![
                            "s3:ObjectCreated:*".to_string(), // TODO pass in (also in builder) + make sure they're valid (enum?)
                        ],
                        arn,
                    }
                ]),
                topic_configs: None,
            }
        } else if let Some(arn) = self.sns_ref {
            NotificationConfiguration {
                topic_configs:  Some(vec![
                    TopicConfiguration {
                        events: vec![
                            "s3:ObjectCreated:*".to_string(), // TODO pass in (also in builder) + make sure they're valid (enum?)
                        ],
                        arn,
                    }
                ]),
                lambda_configs: None,
            }
        } else {
            unreachable!("should be either lambda or sns");
        };

        stack_builder.add_resource(BucketNotification {
            id: self.id,
            resource_id,
            r#type: "Custom::S3BucketNotifications".to_string(),
            properties: BucketNotificationProperties {
                notification_configuration: config,
                service_token: self.handler_arn,
                bucket_name: self.bucket_ref,
                managed: true,
                skip_destination_validation: false,
                depends_on: vec![self.dependency.to_string()],
            },
        });

        bucket_notification_ref
    }
}
