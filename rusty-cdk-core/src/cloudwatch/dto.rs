use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{dto_methods, ref_struct};
use crate::shared::Id;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum LogGroupType {
    #[serde(rename = "AWS::Logs::LogGroup")]
    LogGroupType
}

ref_struct!(LogGroupRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct LogGroup {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: LogGroupType,
    #[serde(rename = "Properties")]
    pub(super) properties: LogGroupProperties,
}
dto_methods!(LogGroup);

#[derive(Debug, Serialize, Deserialize)]
pub struct LogGroupProperties {
    #[serde(rename = "LogGroupClass", skip_serializing_if = "Option::is_none")]
    pub(super) log_group_class: Option<String>,
    #[serde(rename = "LogGroupName", skip_serializing_if = "Option::is_none")]
    pub(super) log_group_name: Option<Value>,
    #[serde(rename = "RetentionInDays", skip_serializing_if = "Option::is_none")]
    pub(super) log_group_retention: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum AlarmType {
    #[serde(rename = "AWS::CloudWatch::Alarm")]
    AlarmType
}

ref_struct!(AlarmRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct Alarm {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: AlarmType,
    #[serde(rename = "Properties")]
    pub(super) properties: AlarmProperties,
}
dto_methods!(Alarm);

#[derive(Debug, Serialize, Deserialize)]
pub struct AlarmProperties {
    #[serde(rename = "ActionsEnabled", skip_serializing_if = "Option::is_none")]
    pub(super) actions_enabled: Option<bool>,
    #[serde(rename = "AlarmActions", skip_serializing_if = "Option::is_none")]
    pub(super) alarm_actions: Option<Vec<Value>>,
    #[serde(rename = "AlarmDescription", skip_serializing_if = "Option::is_none")]
    pub(super) alarm_description: Option<String>,
    #[serde(rename = "AlarmName", skip_serializing_if = "Option::is_none")]
    pub(super) alarm_name: Option<Value>,
    #[serde(rename = "ComparisonOperator")]
    pub(super) comparison_operator: String,
    #[serde(rename = "DatapointsToAlarm", skip_serializing_if = "Option::is_none")]
    pub(super) datapoints_to_alarm: Option<u32>,
    #[serde(rename = "Dimensions", skip_serializing_if = "Option::is_none")]
    pub(super) dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "EvaluateLowSampleCountPercentile", skip_serializing_if = "Option::is_none")]
    pub(super) evaluate_low_sample_count_percentile: Option<String>,
    #[serde(rename = "EvaluationPeriods")]
    pub(super) evaluation_periods: u32,
    #[serde(rename = "ExtendedStatistic", skip_serializing_if = "Option::is_none")]
    pub(super) extended_statistic: Option<String>,
    #[serde(rename = "InsufficientDataActions", skip_serializing_if = "Option::is_none")]
    pub(super) insufficient_data_actions: Option<Vec<Value>>,
    #[serde(rename = "MetricName", skip_serializing_if = "Option::is_none")]
    pub(super) metric_name: Option<String>,
    #[serde(rename = "Metrics", skip_serializing_if = "Option::is_none")]
    pub(super) metrics: Option<Vec<MetricDataQuery>>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub(super) namespace: Option<String>,
    #[serde(rename = "OKActions", skip_serializing_if = "Option::is_none")]
    pub(super) ok_actions: Option<Vec<Value>>,
    #[serde(rename = "Period", skip_serializing_if = "Option::is_none")]
    pub(super) period: Option<u32>,
    #[serde(rename = "Statistic", skip_serializing_if = "Option::is_none")]
    pub(super) statistic: Option<String>,
    #[serde(rename = "Threshold", skip_serializing_if = "Option::is_none")]
    pub(super) threshold: Option<f64>,
    #[serde(rename = "ThresholdMetricId", skip_serializing_if = "Option::is_none")]
    pub(super) threshold_metric_id: Option<String>,
    #[serde(rename = "TreatMissingData", skip_serializing_if = "Option::is_none")]
    pub(super) treat_missing_data: Option<String>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub(super) unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(rename = "Name")]
    pub(super) name: String,
    #[serde(rename = "Value")]
    pub(super) value: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricDataQuery {
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub(super) account_id: Option<String>,
    #[serde(rename = "Expression", skip_serializing_if = "Option::is_none")]
    pub(super) expression: Option<Value>,
    #[serde(rename = "Id")]
    pub(super) id: String,
    #[serde(rename = "Label", skip_serializing_if = "Option::is_none")]
    pub(super) label: Option<String>,
    #[serde(rename = "MetricStat", skip_serializing_if = "Option::is_none")]
    pub(super) metric_stat: Option<MetricStat>,
    #[serde(rename = "Period", skip_serializing_if = "Option::is_none")]
    pub(super) period: Option<u32>,
    #[serde(rename = "ReturnData", skip_serializing_if = "Option::is_none")]
    pub(super) return_data: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricStat {
    #[serde(rename = "Metric")]
    pub(super) metric: Metric,
    #[serde(rename = "Period")]
    pub(super) period: u32,
    #[serde(rename = "Stat")]
    pub(super) stat: String,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub(super) unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    #[serde(rename = "Dimensions", skip_serializing_if = "Option::is_none")]
    pub(super) dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "MetricName")]
    pub(super) metric_name: String,
    #[serde(rename = "Namespace")]
    pub(super) namespace: String,
}

