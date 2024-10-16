// dashboard.jsonnet
local g = import 'github.com/grafana/grafonnet/gen/grafonnet-latest/main.libsonnet';




g.dashboard.new('PromGuard')
+ g.dashboard.withUid('promguard')
+ g.dashboard.withDescription('Dashboard for PromGuard Metrics')
+ g.dashboard.graphTooltip.withSharedCrosshair()
+ g.dashboard.withPanels([
  g.panel.timeSeries.new('Requests / sec')
  + g.panel.timeSeries.queryOptions.withTargets([
    g.query.prometheus.new(
      'mimir',
      'sum by (status_code) (rate(request_duration_seconds_count{job=~".*/faro-api"}[$__rate_interval]))',
    ),
  ])
  + g.panel.timeSeries.standardOptions.withUnit('reqps')
  + g.panel.timeSeries.gridPos.withW(24)
  + g.panel.timeSeries.gridPos.withH(8),
])

// grafonnet.dashboard.new(
//   'Prometheus Metrics Dashboard',
//   time_from='now-1h',
//   refresh='5s',
// ) +
// grafonnet.dashboard.addPanel(
//   grafonnet.row.new('Prometheus Metrics Overview') +
//   grafonnet.row.addPanel(
//     grafonnet.singlestat.new(
//       'Total Count',
//       'sum(label_count_by_label_name)',
//     ) +
//     grafonnet.singlestat.setSpan(4)
//   ) +
//   grafonnet.row.addPanel(
//     grafonnet.table.new(
//       'Metrics Table',
//       'label_count_by_label_name',
//     ) +
//     grafonnet.table.addColumn('instance', 'Instance') +
//     grafonnet.table.addColumn('name', 'Label Name') +
//     grafonnet.table.addColumn('value', 'Count') +
//     grafonnet.table.setSpan(8)
//   ) +
//   grafonnet.row.addPanel(
//     grafonnet.graph.new(
//       'Metrics Distribution',
//       'label_count_by_label_name',
//     ) +
//     grafonnet.graph.addTarget(
//       grafonnet.prometheus.target(
//         'label_count_by_label_name',
//         legendFormat='{{name}}',
//       )
//     ) +
//     grafonnet.graph.setSpan(12)
//   )
// )

