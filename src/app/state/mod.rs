/*! state相关配置 */
use std::sync::RwLock;
use std::{any::Any, collections::HashMap};
use uuid::Uuid;
/// 请求上下文
pub struct RequestContext {
    map: HashMap<String, Box<dyn Any>>,
}
impl RequestContext {
    pub fn new(map: HashMap<String, Box<dyn Any>>) -> RequestContext {
        RequestContext { map }
    }
    pub fn trace(&self) -> &str {
        self.map
            .get("traceId")
            .unwrap()
            .downcast_ref::<String>()
            .unwrap()
            .as_str()
    }
    pub fn init_request_context() -> RequestContext {
        let mut map: HashMap<String, Box<dyn Any>> = HashMap::new();
        map.insert("traceId".to_string(), Box::new(Uuid::new_v4()));
        RequestContext::new(map)
    }
}
/// 请求计数
#[derive(Debug)]
pub struct RequestCounter<'a> {
    counter: RwLock<HashMap<&'a str, u32>>,
}
impl<'a> RequestCounter<'a> {
    pub fn new() -> RequestCounter<'a> {
        RequestCounter {
            counter: RwLock::new(HashMap::new()),
        }
    }
    /// 接口请求计数
    pub fn count(&'a mut self, interface: &'a str) {
        let mut lock = self.counter.write().unwrap();
        lock.entry(interface)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    pub fn display(&self) {
        let lock = self.counter.read().unwrap();
        println!("request counter now is \n {:#?}", lock)
    }
}
