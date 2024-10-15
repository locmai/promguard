use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub prometheus: PrometheusInstances,
}

#[derive(Debug, Deserialize)]
pub struct PrometheusInstances {
    pub instances: Vec<PrometheusInstance>,
}

#[derive(Debug, Deserialize, Default)]
pub struct PrometheusInstance {
    pub name: String,
    pub url: String,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

// Function to provide default value
fn default_limit() -> i32 {
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_default_config_deserialization() {
        let yaml_config = r#"
        prometheus:
          instances:
            - name: "prometheus-1"
              url: "https://prometheus.demo.do.prometheus.io"
        "#;

        let parsed_config: Config = serde_yaml::from_str(yaml_config).unwrap();

        // Validate that the deserialization worked correctly
        assert_eq!(parsed_config.prometheus.instances.len(), 1);
        assert_eq!(parsed_config.prometheus.instances[0].name, "prometheus-1");
        assert_eq!(parsed_config.prometheus.instances[0].url, "https://prometheus.demo.do.prometheus.io");
        assert_eq!(parsed_config.prometheus.instances[0].limit, 10);
    }

    #[tokio::test]
    async fn test_custom_config_deserialization() {
        let yaml_config = r#"
        prometheus:
          instances:
            - name: "prometheus-1"
              url: "https://prometheus.demo.do.prometheus.io"
            - name: "prometheus-1"
              url: "https://prometheus.demo.do.prometheus.io"
              limit: 15
        "#;

        let parsed_config: Config = serde_yaml::from_str(yaml_config).unwrap();

        // Validate that the deserialization worked correctly
        assert_eq!(parsed_config.prometheus.instances.len(), 2);
        assert_eq!(parsed_config.prometheus.instances[1].name, "prometheus-1");
        assert_eq!(parsed_config.prometheus.instances[1].url, "https://prometheus.demo.do.prometheus.io");
        assert_eq!(parsed_config.prometheus.instances[1].limit, 15);
    }

}
