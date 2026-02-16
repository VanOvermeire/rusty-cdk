use crate::shared::Id;
use crate::{dto_methods, ref_struct};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum AlarmType {
    #[serde(rename = "AWS::CloudWatch::Alarm")]
    AlarmType,
}

ref_struct!(AlarmRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct Alarm {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: AlarmType,
    #[serde(rename = "Properties")]
    pub(crate) properties: AlarmProperties,
}
dto_methods!(Alarm);

#[derive(Debug, Serialize, Deserialize)]
pub struct AlarmProperties {
    #[serde(rename = "ThresholdMetricId", skip_serializing_if = "Option::is_none")]
    pub(crate) threshold_metric_id: Option<String>,
    #[serde(rename = "ActionsEnabled", skip_serializing_if = "Option::is_none")]
    pub(crate) actions_enabled: Option<bool>,
    #[serde(rename = "Period", skip_serializing_if = "Option::is_none")]
    pub(crate) period: Option<u32>,
    #[serde(rename = "ExtendedStatistic", skip_serializing_if = "Option::is_none")]
    pub(crate) extended_statistic: Option<String>,
    #[serde(rename = "DatapointsToAlarm", skip_serializing_if = "Option::is_none")]
    pub(crate) datapoints_to_alarm: Option<u32>,
    #[serde(rename = "Statistic", skip_serializing_if = "Option::is_none")]
    pub(crate) statistic: Option<String>,
    #[serde(rename = "MetricName", skip_serializing_if = "Option::is_none")]
    pub(crate) metric_name: Option<String>,
    #[serde(rename = "ComparisonOperator")]
    pub(crate) comparison_operator: String,
    #[serde(rename = "EvaluationPeriods")]
    pub(crate) evaluation_periods: u32,
    #[serde(rename = "Dimensions", skip_serializing_if = "Option::is_none")]
    pub(crate) dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub(crate) namespace: Option<String>,
    #[serde(rename = "AlarmDescription", skip_serializing_if = "Option::is_none")]
    pub(crate) alarm_description: Option<String>,
    #[serde(rename = "EvaluateLowSampleCountPercentile", skip_serializing_if = "Option::is_none")]
    pub(crate) evaluate_low_sample_count_percentile: Option<String>,
    #[serde(rename = "Threshold", skip_serializing_if = "Option::is_none")]
    pub(crate) threshold: Option<u32>,
    #[serde(rename = "TreatMissingData", skip_serializing_if = "Option::is_none")]
    pub(crate) treat_missing_data: Option<String>,
    #[serde(rename = "AlarmName", skip_serializing_if = "Option::is_none")]
    pub(crate) alarm_name: Option<String>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub(crate) unit: Option<String>,
    #[serde(rename = "Metrics", skip_serializing_if = "Option::is_none")]
    pub(crate) metrics: Option<Vec<MetricDataQuery>>,
    #[serde(rename = "AlarmActions", skip_serializing_if = "Option::is_none")]
    pub(crate) alarm_actions: Option<Vec<String>>,
    #[serde(rename = "InsufficientDataActions", skip_serializing_if = "Option::is_none")]
    pub(crate) insufficient_data_actions: Option<Vec<String>>,
    #[serde(rename = "OKActions", skip_serializing_if = "Option::is_none")]
    pub(crate) ok_actions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum AnomalyDetectorType {
    #[serde(rename = "AWS::CloudWatch::AnomalyDetector")]
    AnomalyDetectorType,
}

ref_struct!(AnomalyDetectorRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct AnomalyDetector {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: AnomalyDetectorType,
    #[serde(rename = "Properties")]
    pub(crate) properties: AnomalyDetectorProperties,
}
dto_methods!(AnomalyDetector);

#[derive(Debug, Serialize, Deserialize)]
pub struct AnomalyDetectorProperties {
    #[serde(rename = "Dimensions", skip_serializing_if = "Option::is_none")]
    pub(crate) dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub(crate) namespace: Option<String>,
    #[serde(rename = "MetricName", skip_serializing_if = "Option::is_none")]
    pub(crate) metric_name: Option<String>,
    #[serde(rename = "MetricCharacteristics", skip_serializing_if = "Option::is_none")]
    pub(crate) metric_characteristics: Option<MetricCharacteristics>,
    #[serde(rename = "MetricMathAnomalyDetector", skip_serializing_if = "Option::is_none")]
    pub(crate) metric_math_anomaly_detector: Option<MetricMathAnomalyDetector>,
    #[serde(rename = "Configuration", skip_serializing_if = "Option::is_none")]
    pub(crate) configuration: Option<Configuration>,
    #[serde(rename = "SingleMetricAnomalyDetector", skip_serializing_if = "Option::is_none")]
    pub(crate) single_metric_anomaly_detector: Option<SingleMetricAnomalyDetector>,
    #[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
    pub(crate) stat: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum CompositeAlarmType {
    #[serde(rename = "AWS::CloudWatch::CompositeAlarm")]
    CompositeAlarmType,
}

ref_struct!(CompositeAlarmRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeAlarm {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: CompositeAlarmType,
    #[serde(rename = "Properties")]
    pub(crate) properties: CompositeAlarmProperties,
}
dto_methods!(CompositeAlarm);

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositeAlarmProperties {
    #[serde(rename = "AlarmDescription", skip_serializing_if = "Option::is_none")]
    pub(crate) alarm_description: Option<String>,
    #[serde(rename = "InsufficientDataActions", skip_serializing_if = "Option::is_none")]
    pub(crate) insufficient_data_actions: Option<Vec<String>>,
    #[serde(rename = "OKActions", skip_serializing_if = "Option::is_none")]
    pub(crate) ok_actions: Option<Vec<String>>,
    #[serde(rename = "AlarmName", skip_serializing_if = "Option::is_none")]
    pub(crate) alarm_name: Option<String>,
    #[serde(rename = "ActionsSuppressorWaitPeriod", skip_serializing_if = "Option::is_none")]
    pub(crate) actions_suppressor_wait_period: Option<u32>,
    #[serde(rename = "ActionsSuppressor", skip_serializing_if = "Option::is_none")]
    pub(crate) actions_suppressor: Option<String>,
    #[serde(rename = "AlarmActions", skip_serializing_if = "Option::is_none")]
    pub(crate) alarm_actions: Option<Vec<String>>,
    #[serde(rename = "AlarmRule")]
    pub(crate) alarm_rule: String,
    #[serde(rename = "ActionsSuppressorExtensionPeriod", skip_serializing_if = "Option::is_none")]
    pub(crate) actions_suppressor_extension_period: Option<u32>,
    #[serde(rename = "ActionsEnabled", skip_serializing_if = "Option::is_none")]
    pub(crate) actions_enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum DashboardType {
    #[serde(rename = "AWS::CloudWatch::Dashboard")]
    DashboardType,
}

ref_struct!(DashboardRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct Dashboard {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: DashboardType,
    #[serde(rename = "Properties")]
    pub(crate) properties: DashboardProperties,
}
dto_methods!(Dashboard);

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardProperties {
    #[serde(rename = "DashboardBody")]
    pub(crate) dashboard_body: String,
    #[serde(rename = "DashboardName", skip_serializing_if = "Option::is_none")]
    pub(crate) dashboard_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum InsightRuleType {
    #[serde(rename = "AWS::CloudWatch::InsightRule")]
    InsightRuleType,
}

ref_struct!(InsightRuleRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct InsightRule {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: InsightRuleType,
    #[serde(rename = "Properties")]
    pub(crate) properties: InsightRuleProperties,
}
dto_methods!(InsightRule);

#[derive(Debug, Serialize, Deserialize)]
pub struct InsightRuleProperties {
    #[serde(rename = "ApplyOnTransformedLogs", skip_serializing_if = "Option::is_none")]
    pub(crate) apply_on_transformed_logs: Option<bool>,
    #[serde(rename = "RuleName")]
    pub(crate) rule_name: String,
    #[serde(rename = "RuleBody")]
    pub(crate) rule_body: String,
    #[serde(rename = "RuleState")]
    pub(crate) rule_state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum MetricStreamType {
    #[serde(rename = "AWS::CloudWatch::MetricStream")]
    MetricStreamType,
}

ref_struct!(MetricStreamRef);

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricStream {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: MetricStreamType,
    #[serde(rename = "Properties")]
    pub(crate) properties: MetricStreamProperties,
}
dto_methods!(MetricStream);

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricStreamProperties {
    #[serde(rename = "IncludeLinkedAccountsMetrics", skip_serializing_if = "Option::is_none")]
    pub(crate) include_linked_accounts_metrics: Option<bool>,
    #[serde(rename = "FirehoseArn", skip_serializing_if = "Option::is_none")]
    pub(crate) firehose_arn: Option<String>,
    #[serde(rename = "IncludeFilters", skip_serializing_if = "Option::is_none")]
    pub(crate) include_filters: Option<Vec<MetricStreamFilter>>,
    #[serde(rename = "RoleArn", skip_serializing_if = "Option::is_none")]
    pub(crate) role_arn: Option<String>,
    #[serde(rename = "OutputFormat", skip_serializing_if = "Option::is_none")]
    pub(crate) output_format: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    #[serde(rename = "StatisticsConfigurations", skip_serializing_if = "Option::is_none")]
    pub(crate) statistics_configurations: Option<Vec<MetricStreamStatisticsConfiguration>>,
    #[serde(rename = "ExcludeFilters", skip_serializing_if = "Option::is_none")]
    pub(crate) exclude_filters: Option<Vec<MetricStreamFilter>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(rename = "Value")]
    pub(crate) value: String,
    #[serde(rename = "Name")]
    pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricDataQuery {
    #[serde(rename = "ReturnData", skip_serializing_if = "Option::is_none")]
    pub(crate) return_data: Option<bool>,
    #[serde(rename = "Expression", skip_serializing_if = "Option::is_none")]
    pub(crate) expression: Option<String>,
    #[serde(rename = "Id")]
    pub(crate) id: String,
    #[serde(rename = "Period", skip_serializing_if = "Option::is_none")]
    pub(crate) period: Option<u32>,
    #[serde(rename = "Label", skip_serializing_if = "Option::is_none")]
    pub(crate) label: Option<String>,
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub(crate) account_id: Option<String>,
    #[serde(rename = "MetricStat", skip_serializing_if = "Option::is_none")]
    pub(crate) metric_stat: Option<MetricStat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub(crate) value: String,
    #[serde(rename = "Key")]
    pub(crate) key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(rename = "ExcludedTimeRanges", skip_serializing_if = "Option::is_none")]
    pub(crate) excluded_time_ranges: Option<Vec<Range>>,
    #[serde(rename = "MetricTimeZone", skip_serializing_if = "Option::is_none")]
    pub(crate) metric_time_zone: Option<String>,
}
// TODO encountered a helper with name Dimension but one already exists - check whether they match
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricCharacteristics {
    #[serde(rename = "PeriodicSpikes", skip_serializing_if = "Option::is_none")]
    pub(crate) periodic_spikes: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricMathAnomalyDetector {
    #[serde(rename = "MetricDataQueries", skip_serializing_if = "Option::is_none")]
    pub(crate) metric_data_queries: Option<Vec<MetricDataQuery>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleMetricAnomalyDetector {
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub(crate) namespace: Option<String>,
    #[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
    pub(crate) stat: Option<String>,
    #[serde(rename = "Dimensions", skip_serializing_if = "Option::is_none")]
    pub(crate) dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub(crate) account_id: Option<String>,
    #[serde(rename = "MetricName", skip_serializing_if = "Option::is_none")]
    pub(crate) metric_name: Option<String>,
}
// TODO encountered a helper with name Tag but one already exists - check whether they match
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricStreamFilter {
    #[serde(rename = "MetricNames", skip_serializing_if = "Option::is_none")]
    pub(crate) metric_names: Option<Vec<String>>,
    #[serde(rename = "Namespace")]
    pub(crate) namespace: String,
}
// TODO encountered a helper with name MetricStreamFilter but one already exists - check whether they match
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricStreamStatisticsConfiguration {
    #[serde(rename = "AdditionalStatistics")]
    pub(crate) additional_statistics: Vec<String>,
    #[serde(rename = "IncludeMetrics")]
    pub(crate) include_metrics: Vec<MetricStreamStatisticsMetric>,
}
// TODO encountered a helper with name Tag but one already exists - check whether they match
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricStat {
    #[serde(rename = "Period")]
    pub(crate) period: u32,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub(crate) unit: Option<String>,
    #[serde(rename = "Metric")]
    pub(crate) metric: Metric,
    #[serde(rename = "Stat")]
    pub(crate) stat: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    #[serde(rename = "EndTime")]
    pub(crate) end_time: String,
    #[serde(rename = "StartTime")]
    pub(crate) start_time: String,
}
// TODO encountered a helper with name MetricDataQuery but one already exists - check whether they match// TODO encountered a helper with name Dimension but one already exists - check whether they match
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricStreamStatisticsMetric {
    #[serde(rename = "MetricName")]
    pub(crate) metric_name: String,
    #[serde(rename = "Namespace")]
    pub(crate) namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    #[serde(rename = "MetricName", skip_serializing_if = "Option::is_none")]
    pub(crate) metric_name: Option<String>,
    #[serde(rename = "Dimensions", skip_serializing_if = "Option::is_none")]
    pub(crate) dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub(crate) namespace: Option<String>,
}
// TODO encountered a helper with name MetricStat but one already exists - check whether they match// TODO encountered a helper with name Dimension but one already exists - check whether they match// TODO encountered a helper with name Metric but one already exists - check whether they match// TODO encountered a helper with name Dimension but one already exists - check whether they match
