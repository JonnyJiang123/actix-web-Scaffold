/*! 应用公共配置 */
use std::collections::HashMap;
#[derive(Debug)]
pub struct CommonConfig(HashMap<String, Value>);
use serde_json::{Number, Value};
impl CommonConfig {
    pub async fn common_config() -> CommonConfig {
        let mut config = HashMap::new();
        config.insert("port".to_string(), Value::Number(Number::from(8080)));
        CommonConfig(config.clone())
    }
}
