use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TsdbStatus {
    pub status: String,
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    //pub head_stats: HeadStats,
    #[serde(rename = "seriesCountByMetricName")]
    pub series_count_by_metric_name: Vec<SeriesCount>,
    #[serde(rename = "labelValueCountByLabelName")]
    pub label_count_by_label_name: Vec<SeriesCount>,
    #[serde(rename = "memoryInBytesByLabelName")]
    pub memory_in_bytes_by_label_name: Vec<SeriesCount>,
}

#[derive(Debug, Deserialize)]
pub struct SeriesCount {
    pub name: String,
    pub value: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[tokio::test]
    async fn test_tsdb_status_deserialization() {
        let json_response = r#"
        {
            "status": "success",
            "data": {
                "seriesCountByMetricName": [
                    {
                        "name": "metric_a",
                        "value": 69
                    },
                    {
                        "name": "metric_a",
                        "value": 170
                    }
                ],
                "labelValueCountByLabelName": [
                    {
                        "name": "label_a",
                        "value": 11002
                    },
                    {
                        "name": "label_d",
                        "value": 89
                    }
                ],
                "memoryInBytesByLabelName": [
                    {
                        "name": "metric_a",
                        "value": 69
                    },
                    {
                        "name": "metric_a",
                        "value": 170
                    }
                ]
            }
        }
        "#;

        let parsed_response: TsdbStatus = serde_json::from_str(json_response).unwrap();

        // Validate that the deserialization worked correctly
        assert_eq!(parsed_response.status, "success");
        assert_eq!(parsed_response.data.series_count_by_metric_name.len(), 2);

        let result = &parsed_response.data.series_count_by_metric_name[0];
        assert_eq!(result.name, "metric_a");
        assert_eq!(result.value, 69);
    }
}
