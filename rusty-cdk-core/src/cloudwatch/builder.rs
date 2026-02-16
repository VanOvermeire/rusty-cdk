use crate::cloudwatch::{
    Alarm, AlarmProperties, AlarmRef, AlarmType, AnomalyDetector, AnomalyDetectorProperties, AnomalyDetectorRef, AnomalyDetectorType,
    CompositeAlarm, CompositeAlarmProperties, CompositeAlarmRef, CompositeAlarmType, Dashboard, DashboardProperties, DashboardRef,
    DashboardType, InsightRule, InsightRuleProperties, InsightRuleRef, InsightRuleType, MetricStream, MetricStreamProperties,
    MetricStreamRef, MetricStreamType,
};
use crate::cloudwatch::{
    Configuration, Dimension, Metric, MetricCharacteristics, MetricDataQuery, MetricMathAnomalyDetector, MetricStat, MetricStreamFilter,
    MetricStreamStatisticsConfiguration, MetricStreamStatisticsMetric, Range, SingleMetricAnomalyDetector, Tag,
};
use crate::shared::Id;
use crate::stack::{Resource, StackBuilder};
use serde_json::Value;

pub struct AlarmBuilder {
    id: Id,
    threshold_metric_id: Option<String>, // In an alarm based on an anomaly detection model, this is the ID of the            <code class="code">ANOMALY_DETECTION_BAND</code> function used as the threshold for the            alarm., Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    actions_enabled: Option<bool>, // Indicates whether actions should be executed during any changes to the alarm state. The default is TRUE.
    period: Option<u32>, // The period, in seconds, over which the statistic is applied. This is required for an alarm based on a            metric. Valid values are 10, 20, 30, 60, and any multiple of 60., For an alarm based on a math expression, you can't            specify <code class="code">Period</code>, and instead you use the <code class="code">Metrics</code> parameter., Minimum: 10
    extended_statistic: Option<String>, // The percentile statistic for the metric associated with the alarm. Specify a value between			p0.0 and p100., For an alarm based on a metric, you must specify either <code class="code">Statistic</code>           or <code class="code">ExtendedStatistic</code> but not both., For an alarm based on a math expression, you can't            specify <code class="code">ExtendedStatistic</code>. Instead, you use <code class="code">Metrics</code>.
    datapoints_to_alarm: Option<u32>, // The number of datapoints that must be breaching to trigger the alarm.		           This is used only if you are setting an "M out of N" alarm. In that case, this value is the M, 		           and the value that you set for <code class="code">EvaluationPeriods</code> is the N value.		           For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#alarm-evaluation">Evaluating		           an Alarm</a> in the Amazon CloudWatch User Guide., If you omit this parameter, CloudWatch uses the same value here that you set       for <code class="code">EvaluationPeriods</code>, and the alarm goes to alarm state if that many consecutive        periods are breaching., Minimum: <code class="code">1</code>
    statistic: Option<String>, // The statistic for the metric associated with the alarm, other than percentile.		    For percentile statistics, use <code class="code">ExtendedStatistic</code>., For an alarm based on a metric, you must specify either <code class="code">Statistic</code>       or <code class="code">ExtendedStatistic</code> but not both., For an alarm based on a math expression, you can't            specify <code class="code">Statistic</code>. Instead, you use <code class="code">Metrics</code>., Allowed values: <code class="code">SampleCount | Average | Sum | Minimum | Maximum</code>
    metric_name: Option<String>, // The name of the metric associated with the alarm. This is required for an alarm based on a 		       metric. For an alarm based on a math expression, you use <code class="code">Metrics</code> instead and you can't 		       specify <code class="code">MetricName</code>., Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    comparison_operator: String, // The arithmetic operation to use when comparing the specified			statistic and threshold. The specified statistic value is used as the first operand., Required: Yes, Allowed values: <code class="code">GreaterThanOrEqualToThreshold | GreaterThanThreshold | LessThanThreshold | LessThanOrEqualToThreshold | LessThanLowerOrGreaterThanUpperThreshold | LessThanLowerThreshold | GreaterThanUpperThreshold</code>
    evaluation_periods: u32, // The number of periods over which data is compared to the specified threshold. 		           If you are setting an alarm that requires that a number of consecutive data points be 		           breaching to trigger the alarm, this value specifies that number. If you 		           are setting an "M out of N" alarm, this value is the N, and <code class="code">DatapointsToAlarm</code> 		           is the M., For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#alarm-evaluation">Evaluating           an Alarm</a> in the Amazon CloudWatch User Guide., Required: Yes, Minimum: <code class="code">1</code>
    dimensions: Option<Vec<Dimension>>, // The dimensions for the metric associated with the alarm. For an alarm based on a math expression, you can't            specify <code class="code">Dimensions</code>. Instead, you use <code class="code">Metrics</code>., Maximum: <code class="code">30</code>
    namespace: Option<String>, // The namespace of the metric associated with the alarm. This is required for an alarm based on a            metric. For an alarm based on a math expression, you can't            specify <code class="code">Namespace</code> and you use <code class="code">Metrics</code> instead., For a list of namespaces for metrics from AWS services, see            <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/aws-services-cloudwatch-metrics.html">AWS Services That Publish CloudWatchMetrics.           </a>, Pattern: <code class="code">[^:].*</code>, Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    alarm_description: Option<String>, // The description of the alarm., Minimum: <code class="code">0</code>, Maximum: <code class="code">1024</code>
    evaluate_low_sample_count_percentile: Option<String>, // Used only for alarms based on percentiles. If <code class="code">ignore</code>, the alarm state            does not change during periods with too few data points to be statistically significant.            If <code class="code">evaluate</code> or this parameter is not used, the alarm is always evaluated            and possibly changes state no matter how many data points are available., Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    threshold: Option<u32>,                               // The value to compare with the specified statistic.
    treat_missing_data: Option<String>, // Sets how this alarm is to handle missing data points. Valid values are <code class="code">breaching</code>, <code class="code">notBreaching</code>, <code class="code">ignore</code>,           and <code class="code">missing</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#alarms-and-missing-data">                Configuring How CloudWatchAlarms Treat Missing Data</a> in the             Amazon CloudWatchUser Guide., If you omit this parameter, the default behavior of <code class="code">missing</code> is used., Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    alarm_name: Option<String>, // The name of the alarm. If you don't specify a name, CloudFormation generates a unique physical ID and uses that ID for the alarm name., Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    unit: Option<String>, // The unit of the metric associated with the alarm. Specify this only if you are creating an alarm		           based on a single metric. Do not specify this if you are specifying a <code class="code">Metrics</code> array., You can specify the following values: Seconds, Microseconds, Milliseconds, Bytes, Kilobytes, 		           Megabytes, Gigabytes, Terabytes, Bits, Kilobits, Megabits, Gigabits, Terabits, Percent, Count, 		           Bytes/Second, Kilobytes/Second, Megabytes/Second, Gigabytes/Second, Terabytes/Second, Bits/Second, 		           Kilobits/Second, Megabits/Second, Gigabits/Second, Terabits/Second, Count/Second, or None., Allowed values: <code class="code">Seconds | Microseconds | Milliseconds | Bytes | Kilobytes | Megabytes | Gigabytes | Terabytes | Bits | Kilobits | Megabits | Gigabits | Terabits | Percent | Count | Bytes/Second | Kilobytes/Second | Megabytes/Second | Gigabytes/Second | Terabytes/Second | Bits/Second | Kilobits/Second | Megabits/Second | Gigabits/Second | Terabits/Second | Count/Second | None</code>
    metrics: Option<Vec<MetricDataQuery>>, // An array that enables you to create an alarm based on the result of a metric math expression. Each        item in the array either retrieves a metric or performs a math expression., If you specify the <code class="code">Metrics</code> parameter, you cannot specify <code class="code">MetricName</code>, <code class="code">Dimensions</code>,            <code class="code">Period</code>, <code class="code">Namespace</code>, <code class="code">Statistic</code>, <code class="code">ExtendedStatistic</code>, or <code class="code">Unit</code>.
    alarm_actions: Option<Vec<String>>, // The list of actions to execute when this alarm transitions into an ALARM state from any other state. 			Specify each action as an Amazon Resource Name (ARN). For more information about creating alarms and the actions 			that you can specify, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_PutMetricAlarm.html">PutMetricAlarm</a> in the 		           Amazon CloudWatch API Reference., Maximum: <code class="code">5</code>
    insufficient_data_actions: Option<Vec<String>>, // The actions to execute when this alarm transitions to the            <code class="code">INSUFFICIENT_DATA</code> state from any other state. Each action is specified            as an Amazon Resource Name (ARN)., Maximum: <code class="code">5</code>
    ok_actions: Option<Vec<String>>, // The actions to execute when this alarm transitions to the <code class="code">OK</code> state            from any other state. Each action is specified as an Amazon Resource Name            (ARN)., Maximum: <code class="code">5</code>
}

impl AlarmBuilder {
    pub fn new(id: &str, comparison_operator: String, evaluation_periods: u32) -> Self {
        Self {
            id: Id(id.to_string()),
            threshold_metric_id: None,
            actions_enabled: None,
            period: None,
            extended_statistic: None,
            datapoints_to_alarm: None,
            statistic: None,
            metric_name: None,
            comparison_operator,
            evaluation_periods,
            dimensions: None,
            namespace: None,
            alarm_description: None,
            evaluate_low_sample_count_percentile: None,
            threshold: None,
            treat_missing_data: None,
            alarm_name: None,
            unit: None,
            metrics: None,
            alarm_actions: None,
            insufficient_data_actions: None,
            ok_actions: None,
        }
    }

    pub fn threshold_metric_id(self, threshold_metric_id: String) -> Self {
        Self {
            threshold_metric_id: Some(threshold_metric_id),
            ..self
        }
    }

    pub fn actions_enabled(self, actions_enabled: bool) -> Self {
        Self {
            actions_enabled: Some(actions_enabled),
            ..self
        }
    }

    pub fn period(self, period: u32) -> Self {
        Self {
            period: Some(period),
            ..self
        }
    }

    pub fn extended_statistic(self, extended_statistic: String) -> Self {
        Self {
            extended_statistic: Some(extended_statistic),
            ..self
        }
    }

    pub fn datapoints_to_alarm(self, datapoints_to_alarm: u32) -> Self {
        Self {
            datapoints_to_alarm: Some(datapoints_to_alarm),
            ..self
        }
    }

    pub fn statistic(self, statistic: String) -> Self {
        Self {
            statistic: Some(statistic),
            ..self
        }
    }

    pub fn metric_name(self, metric_name: String) -> Self {
        Self {
            metric_name: Some(metric_name),
            ..self
        }
    }

    pub fn dimensions(self, dimensions: Vec<Dimension>) -> Self {
        Self {
            dimensions: Some(dimensions),
            ..self
        }
    }

    pub fn namespace(self, namespace: String) -> Self {
        Self {
            namespace: Some(namespace),
            ..self
        }
    }

    pub fn alarm_description(self, alarm_description: String) -> Self {
        Self {
            alarm_description: Some(alarm_description),
            ..self
        }
    }

    pub fn evaluate_low_sample_count_percentile(self, evaluate_low_sample_count_percentile: String) -> Self {
        Self {
            evaluate_low_sample_count_percentile: Some(evaluate_low_sample_count_percentile),
            ..self
        }
    }

    pub fn threshold(self, threshold: u32) -> Self {
        Self {
            threshold: Some(threshold),
            ..self
        }
    }

    pub fn treat_missing_data(self, treat_missing_data: String) -> Self {
        Self {
            treat_missing_data: Some(treat_missing_data),
            ..self
        }
    }

    pub fn alarm_name(self, alarm_name: String) -> Self {
        Self {
            alarm_name: Some(alarm_name),
            ..self
        }
    }

    pub fn unit(self, unit: String) -> Self {
        Self { unit: Some(unit), ..self }
    }

    pub fn metrics(self, metrics: Vec<MetricDataQuery>) -> Self {
        Self {
            metrics: Some(metrics),
            ..self
        }
    }

    pub fn alarm_actions(self, alarm_actions: Vec<String>) -> Self {
        Self {
            alarm_actions: Some(alarm_actions),
            ..self
        }
    }

    pub fn insufficient_data_actions(self, insufficient_data_actions: Vec<String>) -> Self {
        Self {
            insufficient_data_actions: Some(insufficient_data_actions),
            ..self
        }
    }

    pub fn ok_actions(self, ok_actions: Vec<String>) -> Self {
        Self {
            ok_actions: Some(ok_actions),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> AlarmRef {
        let resource_id = Resource::generate_id("Alarm");

        let resource = Alarm {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: AlarmType::AlarmType,
            properties: AlarmProperties {
                threshold_metric_id: self.threshold_metric_id,
                actions_enabled: self.actions_enabled,
                period: self.period,
                extended_statistic: self.extended_statistic,
                datapoints_to_alarm: self.datapoints_to_alarm,
                statistic: self.statistic,
                metric_name: self.metric_name,
                comparison_operator: self.comparison_operator,
                evaluation_periods: self.evaluation_periods,
                dimensions: self.dimensions,
                namespace: self.namespace,
                alarm_description: self.alarm_description,
                evaluate_low_sample_count_percentile: self.evaluate_low_sample_count_percentile,
                threshold: self.threshold,
                treat_missing_data: self.treat_missing_data,
                alarm_name: self.alarm_name,
                unit: self.unit,
                metrics: self.metrics,
                alarm_actions: self.alarm_actions,
                insufficient_data_actions: self.insufficient_data_actions,
                ok_actions: self.ok_actions,
            },
        };
        // stack_builder.add_resource(resource); // TODO add to Resource enum to activate!

        AlarmRef::internal_new(resource_id)
    }
}

pub struct AnomalyDetectorBuilder {
    id: Id,
    dimensions: Option<Vec<Dimension>>, // The dimensions of the metric associated with the anomaly detection band.
    namespace: Option<String>,          // The namespace of the metric associated with the anomaly detection band.
    metric_name: Option<String>,        // The name of the metric associated with the anomaly detection band.
    metric_characteristics: Option<MetricCharacteristics>, // Use this object to include parameters to provide information about your metric to CloudWatch to help it build more accurate anomaly detection models.          Currently, it includes the <code class="code">PeriodicSpikes</code> parameter.
    metric_math_anomaly_detector: Option<MetricMathAnomalyDetector>, // The CloudWatch metric math expression for this anomaly detector.
    configuration: Option<Configuration>, // Specifies details about how the anomaly detection model is to be trained, including time ranges to exclude        when training and updating the model. The configuration can also include the time zone to use for the metric.
    single_metric_anomaly_detector: Option<SingleMetricAnomalyDetector>, // The CloudWatch metric and statistic for this anomaly detector.
    stat: Option<String>,                 // The statistic of the metric associated with the anomaly detection band.
}

impl AnomalyDetectorBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: Id(id.to_string()),
            dimensions: None,
            namespace: None,
            metric_name: None,
            metric_characteristics: None,
            metric_math_anomaly_detector: None,
            configuration: None,
            single_metric_anomaly_detector: None,
            stat: None,
        }
    }

    pub fn dimensions(self, dimensions: Vec<Dimension>) -> Self {
        Self {
            dimensions: Some(dimensions),
            ..self
        }
    }

    pub fn namespace(self, namespace: String) -> Self {
        Self {
            namespace: Some(namespace),
            ..self
        }
    }

    pub fn metric_name(self, metric_name: String) -> Self {
        Self {
            metric_name: Some(metric_name),
            ..self
        }
    }

    pub fn metric_characteristics(self, metric_characteristics: MetricCharacteristics) -> Self {
        Self {
            metric_characteristics: Some(metric_characteristics),
            ..self
        }
    }

    pub fn metric_math_anomaly_detector(self, metric_math_anomaly_detector: MetricMathAnomalyDetector) -> Self {
        Self {
            metric_math_anomaly_detector: Some(metric_math_anomaly_detector),
            ..self
        }
    }

    pub fn configuration(self, configuration: Configuration) -> Self {
        Self {
            configuration: Some(configuration),
            ..self
        }
    }

    pub fn single_metric_anomaly_detector(self, single_metric_anomaly_detector: SingleMetricAnomalyDetector) -> Self {
        Self {
            single_metric_anomaly_detector: Some(single_metric_anomaly_detector),
            ..self
        }
    }

    pub fn stat(self, stat: String) -> Self {
        Self { stat: Some(stat), ..self }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> AnomalyDetectorRef {
        let resource_id = Resource::generate_id("AnomalyDetector");

        let resource = AnomalyDetector {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: AnomalyDetectorType::AnomalyDetectorType,
            properties: AnomalyDetectorProperties {
                dimensions: self.dimensions,
                namespace: self.namespace,
                metric_name: self.metric_name,
                metric_characteristics: self.metric_characteristics,
                metric_math_anomaly_detector: self.metric_math_anomaly_detector,
                configuration: self.configuration,
                single_metric_anomaly_detector: self.single_metric_anomaly_detector,
                stat: self.stat,
            },
        };
        // stack_builder.add_resource(resource); // TODO add to Resource enum to activate!

        AnomalyDetectorRef::internal_new(resource_id)
    }
}

pub struct CompositeAlarmBuilder {
    id: Id,
    alarm_description: Option<String>, // The description for the composite alarm., Minimum: <code class="code">0</code>, Maximum: <code class="code">1024</code>
    insufficient_data_actions: Option<Vec<String>>, // The actions to execute when this alarm transitions to the INSUFFICIENT_DATA state from any other state. Each action is specified as an Amazon Resource Name (ARN).             For more information about creating alarms and the actions             that you can specify, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_PutCompositeAlarm.html">PutCompositeAlarm</a> in the             Amazon CloudWatch API Reference., Minimum: <code class="code">1</code>, Maximum: <code class="code">1024 | 5</code>
    ok_actions: Option<Vec<String>>, // The actions to execute when this alarm transitions to the OK state from any other state. Each action is specified as an Amazon Resource Name (ARN).             For more information about creating alarms and the actions             that you can specify, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_PutCompositeAlarm.html">PutCompositeAlarm</a> in the             Amazon CloudWatch API Reference., Minimum: <code class="code">1</code>, Maximum: <code class="code">1024 | 5</code>
    alarm_name: Option<String>, // The name for the composite alarm. This name must be unique within your AWS account., Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    actions_suppressor_wait_period: Option<u32>, // The maximum time         in seconds         that the composite alarm waits        for the suppressor alarm         to go         into the <code class="code">ALARM</code> state.         After this time,         the composite alarm performs its actions., Minimum: <code class="code">0</code>
    actions_suppressor: Option<String>, // Actions will be suppressed             if the suppressor alarm is             in the <code class="code">ALARM</code> state.            <code class="code">ActionsSuppressor</code> can be an AlarmName or an Amazon Resource Name (ARN)             from an existing alarm., Minimum: <code class="code">1</code>, Maximum: <code class="code">1600</code>
    alarm_actions: Option<Vec<String>>, // The actions to execute when this alarm transitions to the ALARM state from any other state. Each action is specified as an Amazon Resource Name (ARN).             For more information about creating alarms and the actions         that you can specify, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_PutCompositeAlarm.html">PutCompositeAlarm</a> in the         Amazon CloudWatch API Reference., Minimum: <code class="code">1</code>, Maximum: <code class="code">1024 | 5</code>
    alarm_rule: String, // An expression that specifies which other alarms are to be evaluated to determine this composite alarm's state. For each             alarm that you reference, you designate a function that specifies whether that alarm needs to be in ALARM state, OK state,             or INSUFFICIENT_DATA state. You can use operators (AND, OR and NOT) to combine multiple functions in a             single expression. You can use parenthesis to logically group the functions in your expression., You can use either alarm names or ARNs to reference the other alarms that are to be evaluated., Functions can include the following:, TRUE and FALSE are useful for testing a complex AlarmRule structure, and for testing your alarm actions., For more information about <code class="code">AlarmRule</code> syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_PutCompositeAlarm.html">PutCompositeAlarm</a> in the         Amazon CloudWatch API Reference., Required: Yes, Minimum: <code class="code">1</code>, Maximum: <code class="code">10240</code>
    actions_suppressor_extension_period: Option<u32>, // The maximum time         in seconds        that the composite alarm waits         after suppressor alarm goes out         of the <code class="code">ALARM</code> state.         After this time,         the composite alarm performs its actions., Minimum: <code class="code">0</code>
    actions_enabled: Option<bool>, // Indicates whether actions should be executed during any changes to the alarm state of the composite alarm. The default is TRUE.
}

impl CompositeAlarmBuilder {
    pub fn new(id: &str, alarm_rule: String) -> Self {
        Self {
            id: Id(id.to_string()),
            alarm_description: None,
            insufficient_data_actions: None,
            ok_actions: None,
            alarm_name: None,
            actions_suppressor_wait_period: None,
            actions_suppressor: None,
            alarm_actions: None,
            alarm_rule,
            actions_suppressor_extension_period: None,
            actions_enabled: None,
        }
    }

    pub fn alarm_description(self, alarm_description: String) -> Self {
        Self {
            alarm_description: Some(alarm_description),
            ..self
        }
    }

    pub fn insufficient_data_actions(self, insufficient_data_actions: Vec<String>) -> Self {
        Self {
            insufficient_data_actions: Some(insufficient_data_actions),
            ..self
        }
    }

    pub fn ok_actions(self, ok_actions: Vec<String>) -> Self {
        Self {
            ok_actions: Some(ok_actions),
            ..self
        }
    }

    pub fn alarm_name(self, alarm_name: String) -> Self {
        Self {
            alarm_name: Some(alarm_name),
            ..self
        }
    }

    pub fn actions_suppressor_wait_period(self, actions_suppressor_wait_period: u32) -> Self {
        Self {
            actions_suppressor_wait_period: Some(actions_suppressor_wait_period),
            ..self
        }
    }

    pub fn actions_suppressor(self, actions_suppressor: String) -> Self {
        Self {
            actions_suppressor: Some(actions_suppressor),
            ..self
        }
    }

    pub fn alarm_actions(self, alarm_actions: Vec<String>) -> Self {
        Self {
            alarm_actions: Some(alarm_actions),
            ..self
        }
    }

    pub fn actions_suppressor_extension_period(self, actions_suppressor_extension_period: u32) -> Self {
        Self {
            actions_suppressor_extension_period: Some(actions_suppressor_extension_period),
            ..self
        }
    }

    pub fn actions_enabled(self, actions_enabled: bool) -> Self {
        Self {
            actions_enabled: Some(actions_enabled),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> CompositeAlarmRef {
        let resource_id = Resource::generate_id("CompositeAlarm");

        let resource = CompositeAlarm {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: CompositeAlarmType::CompositeAlarmType,
            properties: CompositeAlarmProperties {
                alarm_description: self.alarm_description,
                insufficient_data_actions: self.insufficient_data_actions,
                ok_actions: self.ok_actions,
                alarm_name: self.alarm_name,
                actions_suppressor_wait_period: self.actions_suppressor_wait_period,
                actions_suppressor: self.actions_suppressor,
                alarm_actions: self.alarm_actions,
                alarm_rule: self.alarm_rule,
                actions_suppressor_extension_period: self.actions_suppressor_extension_period,
                actions_enabled: self.actions_enabled,
            },
        };
        // stack_builder.add_resource(resource); // TODO add to Resource enum to activate!

        CompositeAlarmRef::internal_new(resource_id)
    }
}

pub struct DashboardBuilder {
    id: Id,
    dashboard_body: String, // The detailed information about the dashboard in JSON format, including the widgets to include and their location			on the dashboard.  This parameter is required., For more information about the syntax, 		       	see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/CloudWatch-Dashboard-Body-Structure.html">Dashboard Body Structure and Syntax</a>., Required: Yes
    dashboard_name: Option<String>, // The name of the dashboard. The name must be between 1 and 255 characters. If you do not specify a name, one will be generated automatically.
}

impl DashboardBuilder {
    pub fn new(id: &str, dashboard_body: String) -> Self {
        Self {
            id: Id(id.to_string()),
            dashboard_body,
            dashboard_name: None,
        }
    }

    pub fn dashboard_name(self, dashboard_name: String) -> Self {
        Self {
            dashboard_name: Some(dashboard_name),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> DashboardRef {
        let resource_id = Resource::generate_id("Dashboard");

        let resource = Dashboard {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: DashboardType::DashboardType,
            properties: DashboardProperties {
                dashboard_body: self.dashboard_body,
                dashboard_name: self.dashboard_name,
            },
        };
        // stack_builder.add_resource(resource); // TODO add to Resource enum to activate!

        DashboardRef::internal_new(resource_id)
    }
}

pub struct InsightRuleBuilder {
    id: Id,
    apply_on_transformed_logs: Option<bool>, // Determines whether the rules is evaluated on transformed versions of logs. Valid values are <code class="code">TRUE</code> and <code class="code">FALSE</code>.
    rule_name: String,                       // The name of the rule., Required: Yes
    rule_body: String, // The definition of the rule, as a JSON object.         For details about the syntax, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/ContributorInsights-RuleSyntax.html">            Contributor Insights Rule Syntax</a> in the Amazon CloudWatch User Guide., Required: Yes
    rule_state: String, // The current state of the rule. Valid values are <code class="code">ENABLED</code> and <code class="code">DISABLED</code>., Required: Yes
}

impl InsightRuleBuilder {
    pub fn new(id: &str, rule_name: String, rule_body: String, rule_state: String) -> Self {
        Self {
            id: Id(id.to_string()),
            apply_on_transformed_logs: None,
            rule_name,
            rule_body,
            rule_state,
        }
    }

    pub fn apply_on_transformed_logs(self, apply_on_transformed_logs: bool) -> Self {
        Self {
            apply_on_transformed_logs: Some(apply_on_transformed_logs),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> InsightRuleRef {
        let resource_id = Resource::generate_id("InsightRule");

        let resource = InsightRule {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: InsightRuleType::InsightRuleType,
            properties: InsightRuleProperties {
                apply_on_transformed_logs: self.apply_on_transformed_logs,
                rule_name: self.rule_name,
                rule_body: self.rule_body,
                rule_state: self.rule_state,
            },
        };
        // stack_builder.add_resource(resource); // TODO add to Resource enum to activate!

        InsightRuleRef::internal_new(resource_id)
    }
}

pub struct MetricStreamBuilder {
    id: Id,
    include_linked_accounts_metrics: Option<bool>, // If you are creating a metric stream in a monitoring account, specify <code class="code">true</code> to include             metrics from source accounts that are linked to this monitoring account, in the metric stream. The default is <code class="code">false</code>., For more information about linking accounts, see         <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Unified-Cross-Account.html">CloudWatch cross-account observability</a>
    firehose_arn: Option<String>, // The ARN of the Amazon Kinesis Firehose delivery stream to use for this metric stream. This             Amazon Kinesis Firehose delivery stream must already exist and must be in the same account as the metric stream., Minimum: <code class="code">20</code>, Maximum: <code class="code">2048</code>
    include_filters: Option<Vec<MetricStreamFilter>>, // If you specify this parameter, the stream sends only the metrics from the metric namespaces that you specify here.             You cannot specify both <code class="code">IncludeFilters</code> and <code class="code">ExcludeFilters</code> in the same metric stream., When you modify the <code class="code">IncludeFilters</code> or <code class="code">ExcludeFilters</code> of an existing metric stream            in any way, the metric stream is effectively restarted, so after such a change you will get             only the datapoints that have a timestamp after the time of the update., Maximum: <code class="code">1000</code>
    role_arn: Option<String>, // The ARN of an IAM role that this metric stream will use to access Amazon Kinesis Firehose             resources. This IAM role must already exist and must be in the same account as the metric stream.             This IAM role must include the <code class="code">firehose:PutRecord</code> and <code class="code">firehose:PutRecordBatch</code>        permissions., Minimum: <code class="code">20</code>, Maximum: <code class="code">2048</code>
    output_format: Option<String>, // The output format for the stream. Valid values are <code class="code">json</code>, <code class="code">opentelemetry1.0</code> and            <code class="code">opentelemetry0.7</code> For more information about metric stream output formats, see             <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-formats.html">                Metric streams output formats</a>., This parameter is required., Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    name: Option<String>, // If you are creating a new metric stream, this is the name for the new stream.             The name must be different than the names of other metric streams in this account and Region., If you are updating a metric stream, specify the name of that stream here., Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    statistics_configurations: Option<Vec<MetricStreamStatisticsConfiguration>>, // By default, a        metric stream always sends the MAX, MIN, SUM, and SAMPLECOUNT statistics for each metric that is streamed.         You can use this parameter to have the metric stream also send additional statistics in the stream. This         array can have up to 100 members., For each entry in this array, you specify one or more metrics and the list of additional statistics to             stream for those metrics. The additional statistics that you can stream depend on the stream's <code class="code">OutputFormat</code>.             If the <code class="code">OutputFormat</code> is <code class="code">json</code>, you can stream any additional statistic that is supported by             CloudWatch, listed in             <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Statistics-definitions.html">CloudWatch statistics definitions</a>. If the <code class="code">OutputFormat</code> is             OpenTelemetry, you can stream percentile statistics., Maximum: <code class="code">100</code>
    exclude_filters: Option<Vec<MetricStreamFilter>>, // If you specify this parameter, the stream sends metrics from all metric namespaces except             for the namespaces that you specify here. You cannot specify both <code class="code">IncludeFilters</code>             and <code class="code">ExcludeFilters</code> in the same metric stream., When you modify the <code class="code">IncludeFilters</code> or <code class="code">ExcludeFilters</code> of an existing metric stream            in any way, the metric stream is effectively restarted, so after such a change you will get             only the datapoints that have a timestamp after the time of the update., Maximum: <code class="code">1000</code>
}

impl MetricStreamBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: Id(id.to_string()),
            include_linked_accounts_metrics: None,
            firehose_arn: None,
            include_filters: None,
            role_arn: None,
            output_format: None,
            name: None,
            statistics_configurations: None,
            exclude_filters: None,
        }
    }

    pub fn include_linked_accounts_metrics(self, include_linked_accounts_metrics: bool) -> Self {
        Self {
            include_linked_accounts_metrics: Some(include_linked_accounts_metrics),
            ..self
        }
    }

    pub fn firehose_arn(self, firehose_arn: String) -> Self {
        Self {
            firehose_arn: Some(firehose_arn),
            ..self
        }
    }

    pub fn include_filters(self, include_filters: Vec<MetricStreamFilter>) -> Self {
        Self {
            include_filters: Some(include_filters),
            ..self
        }
    }

    pub fn role_arn(self, role_arn: String) -> Self {
        Self {
            role_arn: Some(role_arn),
            ..self
        }
    }

    pub fn output_format(self, output_format: String) -> Self {
        Self {
            output_format: Some(output_format),
            ..self
        }
    }

    pub fn name(self, name: String) -> Self {
        Self { name: Some(name), ..self }
    }

    pub fn statistics_configurations(self, statistics_configurations: Vec<MetricStreamStatisticsConfiguration>) -> Self {
        Self {
            statistics_configurations: Some(statistics_configurations),
            ..self
        }
    }

    pub fn exclude_filters(self, exclude_filters: Vec<MetricStreamFilter>) -> Self {
        Self {
            exclude_filters: Some(exclude_filters),
            ..self
        }
    }

    pub fn build(self, stack_builder: &mut StackBuilder) -> MetricStreamRef {
        let resource_id = Resource::generate_id("MetricStream");

        let resource = MetricStream {
            id: self.id,
            resource_id: resource_id.clone(),
            r#type: MetricStreamType::MetricStreamType,
            properties: MetricStreamProperties {
                include_linked_accounts_metrics: self.include_linked_accounts_metrics,
                firehose_arn: self.firehose_arn,
                include_filters: self.include_filters,
                role_arn: self.role_arn,
                output_format: self.output_format,
                name: self.name,
                statistics_configurations: self.statistics_configurations,
                exclude_filters: self.exclude_filters,
            },
        };
        // stack_builder.add_resource(resource); // TODO add to Resource enum to activate!

        MetricStreamRef::internal_new(resource_id)
    }
}

pub struct DimensionBuilder {
    value: String, // The value for the dimension, from 1–255 characters in length., Required: Yes, Minimum: <code class="code">1</code>, Maximum: <code class="code">1024</code>
    name: String, // The name of the dimension, from 1–255 characters in length. This dimension           name must have been included when           the metric was published., Required: Yes, Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
}

impl DimensionBuilder {
    pub fn new(value: String, name: String) -> Self {
        Self { value, name }
    }

    pub fn build(self) -> Dimension {
        Dimension {
            value: self.value,
            name: self.name,
        }
    }
}

pub struct MetricDataQueryBuilder {
    return_data: Option<bool>, // This option indicates whether to return the			timestamps and raw data values of this metric., When you create an alarm based on a metric math expression, specify <code class="code">True</code> for       this value for only the one math expression that the alarm is based on. You must specify        <code class="code">False</code> for <code class="code">ReturnData</code> for all the other metrics and expressions       used in the alarm., This field is required.
    expression: Option<String>, // The math expression to be performed on the returned data, if this object is performing a math expression. This expression			can use the <code class="code">Id</code> of the other metrics to refer to those metrics, and can also use the <code class="code">Id</code> of other 			expressions to use the result of those expressions. For more information about metric math expressions, see 			<a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/using-metric-math.html#metric-math-syntax">Metric Math Syntax and Functions</a> in the			Amazon CloudWatch User Guide., Within each MetricDataQuery object, you must specify either 			<code class="code">Expression</code> or <code class="code">MetricStat</code> but not both., Minimum: <code class="code">1</code>, Maximum: <code class="code">2048</code>
    id: String, // A short name used to tie this object to the results in the response. This name must be            unique within a single call to <code class="code">GetMetricData</code>. If you are performing math            expressions on this set of data, this name represents that data and can serve as a            variable in the mathematical expression. The valid characters are letters, numbers, and            underscore. The first character must be a lowercase letter., Required: Yes, Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    period: Option<u32>, // The granularity, in seconds, of the returned data points. For metrics with regular            resolution, a period can be as short as one minute (60 seconds) and must be a multiple            of 60. For high-resolution metrics that are collected at intervals of less than one            minute, the period can be 1, 5, 10, 20, 30, 60, or any multiple of 60. High-resolution            metrics are those metrics stored by a <code class="code">PutMetricData</code> operation that includes            a <code class="code">StorageResolution of 1 second</code>., Minimum: <code class="code">1</code>
    label: Option<String>, // A human-readable label for this metric or expression. This is especially useful if this is an expression, so that you know			what the value represents. If the metric or expression is shown in a CloudWatch dashboard widget, the label is shown. If <code class="code">Label</code> is omitted, CloudWatch 			generates a default.
    account_id: Option<String>, // The ID of the account where the metrics are located, if this is a cross-account alarm.
    metric_stat: Option<MetricStat>, // The metric to be returned, along with statistics, period, and units. Use this            parameter only if this object is retrieving a metric and not performing a math            expression on returned data., Within one MetricDataQuery object, you must specify either <code class="code">Expression</code> or            <code class="code">MetricStat</code> but not both.
}

impl MetricDataQueryBuilder {
    pub fn new(id: String) -> Self {
        Self {
            return_data: None,
            expression: None,
            id,
            period: None,
            label: None,
            account_id: None,
            metric_stat: None,
        }
    }

    pub fn return_data(self, return_data: bool) -> Self {
        Self {
            return_data: Some(return_data),
            ..self
        }
    }

    pub fn expression(self, expression: String) -> Self {
        Self {
            expression: Some(expression),
            ..self
        }
    }

    pub fn period(self, period: u32) -> Self {
        Self {
            period: Some(period),
            ..self
        }
    }

    pub fn label(self, label: String) -> Self {
        Self {
            label: Some(label),
            ..self
        }
    }

    pub fn account_id(self, account_id: String) -> Self {
        Self {
            account_id: Some(account_id),
            ..self
        }
    }

    pub fn metric_stat(self, metric_stat: MetricStat) -> Self {
        Self {
            metric_stat: Some(metric_stat),
            ..self
        }
    }

    pub fn build(self) -> MetricDataQuery {
        MetricDataQuery {
            return_data: self.return_data,
            expression: self.expression,
            id: self.id,
            period: self.period,
            label: self.label,
            account_id: self.account_id,
            metric_stat: self.metric_stat,
        }
    }
}

pub struct TagBuilder {
    value: String, // The value for the specified tag key., Required: Yes, Minimum: <code class="code">1</code>, Maximum: <code class="code">256</code>
    key: String, // A string that you can use to assign a value. The combination of tag keys and values            can help you organize and categorize your resources., Required: Yes, Minimum: <code class="code">1</code>, Maximum: <code class="code">128</code>
}

impl TagBuilder {
    pub fn new(value: String, key: String) -> Self {
        Self { value, key }
    }

    pub fn build(self) -> Tag {
        Tag {
            value: self.value,
            key: self.key,
        }
    }
}

pub struct ConfigurationBuilder {
    excluded_time_ranges: Option<Vec<Range>>, // Specifies an array of time ranges to exclude from use when the anomaly detection model is trained and updated.             Use this to make sure that events that could cause unusual values for the metric, such as deployments, aren't used when             CloudWatch creates or updates the model.
    metric_time_zone: Option<String>, // The time zone to use for the metric. This is useful to enable the model to automatically account for daylight savings            time changes if the metric is sensitive to such time changes., To specify a time zone, use the name of the time zone as specified in the standard tz database. For more information,                 see <a href="https://en.wikipedia.org/wiki/Tz_database" rel="noopener noreferrer" target="_blank"><span>tz database</span><awsui-icon class="awsdocs-link-icon" name="external"></awsui-icon></a>.
}

impl ConfigurationBuilder {
    pub fn new() -> Self {
        Self {
            excluded_time_ranges: None,
            metric_time_zone: None,
        }
    }

    pub fn excluded_time_ranges(self, excluded_time_ranges: Vec<Range>) -> Self {
        Self {
            excluded_time_ranges: Some(excluded_time_ranges),
            ..self
        }
    }

    pub fn metric_time_zone(self, metric_time_zone: String) -> Self {
        Self {
            metric_time_zone: Some(metric_time_zone),
            ..self
        }
    }

    pub fn build(self) -> Configuration {
        Configuration {
            excluded_time_ranges: self.excluded_time_ranges,
            metric_time_zone: self.metric_time_zone,
        }
    }
}

pub struct MetricCharacteristicsBuilder {
    periodic_spikes: Option<bool>, // Set this parameter to true if values for this metric consistently include spikes that should not be considered to be anomalies. With this set to true,          CloudWatch will expect to see spikes that occurred consistently during the model training period, and won't flag future similar spikes as anomalies.
}

impl MetricCharacteristicsBuilder {
    pub fn new() -> Self {
        Self { periodic_spikes: None }
    }

    pub fn periodic_spikes(self, periodic_spikes: bool) -> Self {
        Self {
            periodic_spikes: Some(periodic_spikes),
            ..self
        }
    }

    pub fn build(self) -> MetricCharacteristics {
        MetricCharacteristics {
            periodic_spikes: self.periodic_spikes,
        }
    }
}

pub struct MetricMathAnomalyDetectorBuilder {
    metric_data_queries: Option<Vec<MetricDataQuery>>, // An array of metric data query structures that enables you to create an anomaly            detector based on the result of a metric math expression. Each item in            <code class="code">MetricDataQueries</code> gets a metric or performs a math expression. One item            in <code class="code">MetricDataQueries</code> is the expression that provides the time series that            the anomaly detector uses as input. Designate the expression by setting            <code class="code">ReturnData</code> to <code class="code">true</code> for this object in the array. For all            other expressions and metrics, set <code class="code">ReturnData</code> to <code class="code">false</code>. The            designated expression must return a single time series.
}

impl MetricMathAnomalyDetectorBuilder {
    pub fn new() -> Self {
        Self { metric_data_queries: None }
    }

    pub fn metric_data_queries(self, metric_data_queries: Vec<MetricDataQuery>) -> Self {
        Self {
            metric_data_queries: Some(metric_data_queries),
            ..self
        }
    }

    pub fn build(self) -> MetricMathAnomalyDetector {
        MetricMathAnomalyDetector {
            metric_data_queries: self.metric_data_queries,
        }
    }
}

pub struct SingleMetricAnomalyDetectorBuilder {
    namespace: Option<String>, // The namespace of the metric to create the anomaly detection model for., Pattern: <code class="code">[^:].*</code>, Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    stat: Option<String>, // The statistic to use for the metric and anomaly detection model., Pattern: <code class="code">(SampleCount|Average|Sum|Minimum|Maximum|IQM|(p|tc|tm|ts|wm)(\d<span>{</span>1,2}(\.\d<span>{</span>0,10})?|100)|[ou]\d+(\.\d*)?)(_E|_L|_H)?|(TM|TC|TS|WM)\(((((\d<span>{</span>1,2})(\.\d<span>{</span>0,10})?|100(\.0<span>{</span>0,10})?)%)?:((\d<span>{</span>1,2})(\.\d<span>{</span>0,10})?|100(\.0<span>{</span>0,10})?)%|((\d<span>{</span>1,2})(\.\d<span>{</span>0,10})?|100(\.0<span>{</span>0,10})?)%:(((\d<span>{</span>1,2})(\.\d<span>{</span>0,10})?|100(\.0<span>{</span>0,10})?)%)?)\)|(TM|TC|TS|WM|PR)\(((\d+(\.\d<span>{</span>0,10})?|(\d+(\.\d<span>{</span>0,10})?[Ee][+-]?\d+)):((\d+(\.\d<span>{</span>0,10})?|(\d+(\.\d<span>{</span>0,10})?[Ee][+-]?\d+)))?|((\d+(\.\d<span>{</span>0,10})?|(\d+(\.\d<span>{</span>0,10})?[Ee][+-]?\d+)))?:(\d+(\.\d<span>{</span>0,10})?|(\d+(\.\d<span>{</span>0,10})?[Ee][+-]?\d+)))\)</code>, Maximum: <code class="code">50</code>
    dimensions: Option<Vec<Dimension>>, // The metric dimensions to create the anomaly detection model for., Maximum: <code class="code">30</code>
    account_id: Option<String>, // If the CloudWatch metric that provides the time series that the anomaly detector uses as input is in another account, specify that account ID here. If you omit this parameter, the current account is used.
    metric_name: Option<String>, // The name of the metric to create the anomaly detection model for., Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
}

impl SingleMetricAnomalyDetectorBuilder {
    pub fn new() -> Self {
        Self {
            namespace: None,
            stat: None,
            dimensions: None,
            account_id: None,
            metric_name: None,
        }
    }

    pub fn namespace(self, namespace: String) -> Self {
        Self {
            namespace: Some(namespace),
            ..self
        }
    }

    pub fn stat(self, stat: String) -> Self {
        Self { stat: Some(stat), ..self }
    }

    pub fn dimensions(self, dimensions: Vec<Dimension>) -> Self {
        Self {
            dimensions: Some(dimensions),
            ..self
        }
    }

    pub fn account_id(self, account_id: String) -> Self {
        Self {
            account_id: Some(account_id),
            ..self
        }
    }

    pub fn metric_name(self, metric_name: String) -> Self {
        Self {
            metric_name: Some(metric_name),
            ..self
        }
    }

    pub fn build(self) -> SingleMetricAnomalyDetector {
        SingleMetricAnomalyDetector {
            namespace: self.namespace,
            stat: self.stat,
            dimensions: self.dimensions,
            account_id: self.account_id,
            metric_name: self.metric_name,
        }
    }
}

pub struct MetricStreamFilterBuilder {
    metric_names: Option<Vec<String>>, // The names of the metrics to either include or exclude from the metric stream., If you omit this parameter, all metrics in the namespace are included or excluded, depending on whether this                 filter is specified as an exclude filter or an include filter., Each metric name can contain only ASCII printable characters (ASCII range 32 through 126). Each metric name                must contain at least one non-whitespace character., Minimum: <code class="code">1</code>, Maximum: <code class="code">255 | 999</code>
    namespace: String, // The name of the metric namespace in the filter., The namespace can contain only ASCII printable characters (ASCII range 32 through 126). It must             contain at least one non-whitespace character., Required: Yes, Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
}

impl MetricStreamFilterBuilder {
    pub fn new(namespace: String) -> Self {
        Self {
            metric_names: None,
            namespace,
        }
    }

    pub fn metric_names(self, metric_names: Vec<String>) -> Self {
        Self {
            metric_names: Some(metric_names),
            ..self
        }
    }

    pub fn build(self) -> MetricStreamFilter {
        MetricStreamFilter {
            metric_names: self.metric_names,
            namespace: self.namespace,
        }
    }
}

pub struct MetricStreamStatisticsConfigurationBuilder {
    additional_statistics: Vec<String>, // The        additional statistics to stream for the metrics listed in <code class="code">IncludeMetrics</code>., Required: Yes, Maximum: <code class="code">20</code>
    include_metrics: Vec<MetricStreamStatisticsMetric>, // An array that    defines the metrics that are to have additional statistics streamed., Required: Yes, Maximum: <code class="code">100</code>
}

impl MetricStreamStatisticsConfigurationBuilder {
    pub fn new(additional_statistics: Vec<String>, include_metrics: Vec<MetricStreamStatisticsMetric>) -> Self {
        Self {
            additional_statistics,
            include_metrics,
        }
    }

    pub fn build(self) -> MetricStreamStatisticsConfiguration {
        MetricStreamStatisticsConfiguration {
            additional_statistics: self.additional_statistics,
            include_metrics: self.include_metrics,
        }
    }
}

pub struct MetricStatBuilder {
    period: u32, // The granularity, in seconds, of the returned data points. For metrics with regular            resolution, a period can be as short as one minute (60 seconds) and must be a multiple            of 60. For high-resolution metrics that are collected at intervals of less than one            minute, the period can be 1, 5, 10, 20, 30, 60, or any multiple of 60. High-resolution            metrics are those metrics stored by a <code class="code">PutMetricData</code> call that includes a            <code class="code">StorageResolution</code> of 1 second., If the <code class="code">StartTime</code> parameter specifies a time stamp that is greater than            3 hours ago, you must specify the period as follows or no data points in that time range            is returned:, Required: Yes, Minimum: <code class="code">1</code>
    unit: Option<String>, // The unit to use for the returned data points., Valid values are: Seconds, Microseconds, Milliseconds, Bytes, Kilobytes,            Megabytes, Gigabytes, Terabytes, Bits, Kilobits, Megabits, Gigabits, Terabits, Percent, Count,            Bytes/Second, Kilobytes/Second, Megabytes/Second, Gigabytes/Second, Terabytes/Second, Bits/Second,            Kilobits/Second, Megabits/Second, Gigabits/Second, Terabits/Second, Count/Second, or None., Allowed values: <code class="code">Seconds | Microseconds | Milliseconds | Bytes | Kilobytes | Megabytes | Gigabytes | Terabytes | Bits | Kilobits | Megabits | Gigabits | Terabits | Percent | Count | Bytes/Second | Kilobytes/Second | Megabytes/Second | Gigabytes/Second | Terabytes/Second | Bits/Second | Kilobits/Second | Megabits/Second | Gigabits/Second | Terabits/Second | Count/Second | None</code>
    metric: Metric,       // The metric to return, including the metric name, namespace, and dimensions., Required: Yes
    stat: String, // The statistic to return. It can include any CloudWatch statistic or extended statistic.		           For a list of valid values, see the table in <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#Statistic">		               Statistics</a> in the Amazon CloudWatch User Guide., Required: Yes
}

impl MetricStatBuilder {
    pub fn new(period: u32, metric: Metric, stat: String) -> Self {
        Self {
            period,
            unit: None,
            metric,
            stat,
        }
    }

    pub fn unit(self, unit: String) -> Self {
        Self { unit: Some(unit), ..self }
    }

    pub fn build(self) -> MetricStat {
        MetricStat {
            period: self.period,
            unit: self.unit,
            metric: self.metric,
            stat: self.stat,
        }
    }
}

pub struct RangeBuilder {
    end_time: String, // The end time of the range to exclude. The format is <code class="code">yyyy-MM-dd'T'HH:mm:ss</code>. For example,                 <code class="code">2019-07-01T23:59:59</code>., Required: Yes
    start_time: String, // The start time of the range to exclude. The format is <code class="code">yyyy-MM-dd'T'HH:mm:ss</code>. For example,                 <code class="code">2019-07-01T23:59:59</code>., Required: Yes
}

impl RangeBuilder {
    pub fn new(end_time: String, start_time: String) -> Self {
        Self { end_time, start_time }
    }

    pub fn build(self) -> Range {
        Range {
            end_time: self.end_time,
            start_time: self.start_time,
        }
    }
}

pub struct MetricStreamStatisticsMetricBuilder {
    metric_name: String, // The name of the metric., Required: Yes, Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    namespace: String, // The namespace of the metric., Required: Yes, Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
}

impl MetricStreamStatisticsMetricBuilder {
    pub fn new(metric_name: String, namespace: String) -> Self {
        Self { metric_name, namespace }
    }

    pub fn build(self) -> MetricStreamStatisticsMetric {
        MetricStreamStatisticsMetric {
            metric_name: self.metric_name,
            namespace: self.namespace,
        }
    }
}

pub struct MetricBuilder {
    metric_name: Option<String>, // The name of the metric that you want the alarm to watch. This is a required field., Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
    dimensions: Option<Vec<Dimension>>, // The metric dimensions that you want to be used for the metric that		           the alarm will watch., Maximum: <code class="code">30</code>
    namespace: Option<String>, // The namespace of the metric that the alarm will watch., Pattern: <code class="code">[^:].*</code>, Minimum: <code class="code">1</code>, Maximum: <code class="code">255</code>
}

impl MetricBuilder {
    pub fn new() -> Self {
        Self {
            metric_name: None,
            dimensions: None,
            namespace: None,
        }
    }

    pub fn metric_name(self, metric_name: String) -> Self {
        Self {
            metric_name: Some(metric_name),
            ..self
        }
    }

    pub fn dimensions(self, dimensions: Vec<Dimension>) -> Self {
        Self {
            dimensions: Some(dimensions),
            ..self
        }
    }

    pub fn namespace(self, namespace: String) -> Self {
        Self {
            namespace: Some(namespace),
            ..self
        }
    }

    pub fn build(self) -> Metric {
        Metric {
            metric_name: self.metric_name,
            dimensions: self.dimensions,
            namespace: self.namespace,
        }
    }
}
