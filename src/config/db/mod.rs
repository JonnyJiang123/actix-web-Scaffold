/*! db相关配置 */
use std::collections::HashMap;
pub struct MysqlPool(HashMap<String, String>);
impl MysqlPool {
    pub async fn config_mysql() -> MysqlPool {
        MysqlPool(HashMap::new())
    }
}
pub struct RedisConfig(HashMap<String, String>);
impl RedisConfig {
    pub async fn config_redis() -> RedisConfig {
        RedisConfig(HashMap::new())
    }
}
