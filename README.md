## PromGuard

Watch over Prometheus TSDB API

```
cargo run -- --config ./src/promguard-config.yaml

curl localhost:8080/metrics

# HELP status TSDB Status.
# TYPE status gauge
status{instance="prometheus-2",name="status"} 0
status{instance="prometheus-1",name="status"} 0
# HELP series_count_by_metric_name Series Count by Metric Name.
# TYPE series_count_by_metric_name gauge
series_count_by_metric_name{instance="prometheus-2",name="grafana_alerting_request_duration_seconds_bucket"} 132
series_count_by_metric_name{instance="prometheus-2",name="caddy_http_response_size_bytes_bucket"} 432
series_count_by_metric_name{instance="prometheus-2",name="rpc_durations_histogram_seconds_bucket"} 84
series_count_by_metric_name{instance="prometheus-1",name="caddy_http_request_size_bytes_bucket"} 432
series_count_by_metric_name{instance="prometheus-1",name="container_tasks_state"} 235
series_count_by_metric_name{instance="prometheus-2",name="node_systemd_unit_state"} 980
series_count_by_metric_name{instance="prometheus-2",name="caddy_http_request_duration_seconds_bucket"} 576
series_count_by_metric_name{instance="prometheus-2",name="prometheus_http_request_duration_seconds_bucket"} 400
series_count_by_metric_name{instance="prometheus-2",name="container_memory_failures_total"} 188
series_count_by_metric_name{instance="prometheus-1",name="caddy_http_request_duration_seconds_bucket"} 576
series_count_by_metric_name{instance="prometheus-2",name="caddy_http_response_duration_seconds_bucket"} 516
series_count_by_metric_name{instance="prometheus-1",name="prometheus_http_response_size_bytes_bucket"} 360
series_count_by_metric_name{instance="prometheus-1",name="caddy_http_response_duration_seconds_bucket"} 516
series_count_by_metric_name{instance="prometheus-2",name="caddy_http_request_size_bytes_bucket"} 432
series_count_by_metric_name{instance="prometheus-1",name="prometheus_http_request_duration_seconds_bucket"} 400
series_count_by_metric_name{instance="prometheus-1",name="caddy_http_response_size_bytes_bucket"} 432
series_count_by_metric_name{instance="prometheus-2",name="container_tasks_state"} 235
series_count_by_metric_name{instance="prometheus-2",name="prometheus_http_response_size_bytes_bucket"} 360
series_count_by_metric_name{instance="prometheus-1",name="node_systemd_unit_state"} 980
series_count_by_metric_name{instance="prometheus-2",name="grafana_feature_toggles_info"} 179
series_count_by_metric_name{instance="prometheus-1",name="container_memory_failures_total"} 188
series_count_by_metric_name{instance="prometheus-2",name="alertmanager_http_request_duration_seconds_bucket"} 88
series_count_by_metric_name{instance="prometheus-2",name="grafana_http_request_duration_seconds_bucket"} 793
series_count_by_metric_name{instance="prometheus-2",name="alertmanager_notification_latency_seconds_bucket"} 78
series_count_by_metric_name{instance="prometheus-1",name="grafana_http_request_duration_seconds_bucket"} 793
# HELP label_count_by_label_name Label Count by Metric Name.
# TYPE label_count_by_label_name gauge
label_count_by_label_name{instance="prometheus-2",name="service"} 14
label_count_by_label_name{instance="prometheus-2",name="version"} 16
label_count_by_label_name{instance="prometheus-2",name="action"} 14
label_count_by_label_name{instance="prometheus-2",name="handler"} 100
label_count_by_label_name{instance="prometheus-1",name="name"} 379
label_count_by_label_name{instance="prometheus-2",name="integration"} 13
label_count_by_label_name{instance="prometheus-1",name="method"} 41
label_count_by_label_name{instance="prometheus-1",name="handler"} 100
label_count_by_label_name{instance="prometheus-2",name="collector"} 47
label_count_by_label_name{instance="prometheus-2",name="state"} 18
label_count_by_label_name{instance="prometheus-2",name="name"} 379
label_count_by_label_name{instance="prometheus-1",name="le"} 217
label_count_by_label_name{instance="prometheus-2",name="id"} 48
label_count_by_label_name{instance="prometheus-2",name="__name__"} 1100
label_count_by_label_name{instance="prometheus-1",name="state"} 18
label_count_by_label_name{instance="prometheus-1",name="device"} 19
label_count_by_label_name{instance="prometheus-1",name="id"} 48
label_count_by_label_name{instance="prometheus-1",name="__name__"} 1100
label_count_by_label_name{instance="prometheus-1",name="code"} 22
label_count_by_label_name{instance="prometheus-1",name="collector"} 47
label_count_by_label_name{instance="prometheus-2",name="code"} 22
label_count_by_label_name{instance="prometheus-2",name="device"} 19
label_count_by_label_name{instance="prometheus-2",name="type"} 14
label_count_by_label_name{instance="prometheus-2",name="method"} 41
label_count_by_label_name{instance="prometheus-2",name="le"} 217
# HELP memory_in_bytes_by_label_name Memory in Bytes by Label Name.
# TYPE memory_in_bytes_by_label_name gauge
memory_in_bytes_by_label_name{instance="prometheus-2",name="state"} 10663
memory_in_bytes_by_label_name{instance="prometheus-2",name="__name__"} 390284
memory_in_bytes_by_label_name{instance="prometheus-1",name="instance"} 245938
memory_in_bytes_by_label_name{instance="prometheus-1",name="id"} 51397
memory_in_bytes_by_label_name{instance="prometheus-2",name="name"} 26855
memory_in_bytes_by_label_name{instance="prometheus-1",name="job"} 73639
memory_in_bytes_by_label_name{instance="prometheus-1",name="state"} 10663
memory_in_bytes_by_label_name{instance="prometheus-2",name="slo_group"} 8160
memory_in_bytes_by_label_name{instance="prometheus-2",name="id"} 51397
memory_in_bytes_by_label_name{instance="prometheus-1",name="env"} 14888
memory_in_bytes_by_label_name{instance="prometheus-2",name="job"} 73639
memory_in_bytes_by_label_name{instance="prometheus-2",name="le"} 23004
memory_in_bytes_by_label_name{instance="prometheus-2",name="code"} 7415
memory_in_bytes_by_label_name{instance="prometheus-2",name="route"} 5278
memory_in_bytes_by_label_name{instance="prometheus-1",name="__name__"} 390284
memory_in_bytes_by_label_name{instance="prometheus-2",name="env"} 14888
memory_in_bytes_by_label_name{instance="prometheus-1",name="method"} 14650
memory_in_bytes_by_label_name{instance="prometheus-1",name="le"} 23004
memory_in_bytes_by_label_name{instance="prometheus-2",name="instance"} 245938
memory_in_bytes_by_label_name{instance="prometheus-2",name="server"} 9332
memory_in_bytes_by_label_name{instance="prometheus-1",name="handler"} 52423
memory_in_bytes_by_label_name{instance="prometheus-2",name="handler"} 52423
memory_in_bytes_by_label_name{instance="prometheus-2",name="status_source"} 6198
memory_in_bytes_by_label_name{instance="prometheus-1",name="name"} 26855
memory_in_bytes_by_label_name{instance="prometheus-2",name="method"} 14650
# EOF
```
