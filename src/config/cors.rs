use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    pub allowed_origin: String,
    pub allowed_methods: Vec<HttpMethod>,
    pub max_age: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method_deserialize() {
        let json = r#""GET""#;
        let method: HttpMethod = serde_json::from_str(json).unwrap();
        assert_eq!(method, HttpMethod::Get);
    }

    #[test]
    fn test_cors_config_deserialize() {
        let json = r#"
        {
            "allowed_origin": "http://localhost:3000",
            "allowed_methods": ["GET", "POST"],
            "max_age": 3600
        }
        "#;

        let config: CorsConfig = serde_json::from_str(json).unwrap();

        assert_eq!(config.allowed_origin, "http://localhost:3000");
        assert_eq!(config.allowed_methods.len(), 2);
        assert_eq!(config.max_age, 3600);
    }
}
