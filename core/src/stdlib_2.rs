use crate::types::VelvetValue;
use chrono::Utc;

pub fn json_parse(json: &str) -> VelvetValue {
    VelvetValue::new("string", &format!("Parsed JSON: {}", json))
}

pub fn json_stringify(data: &str) -> VelvetValue {
    VelvetValue::new("string", &format!("Stringified JSON: {}", data))
}

pub fn ai_predict(data: &str) -> VelvetValue {
    VelvetValue::new("string", &format!("AI prediction: {}", data))
}

pub fn uuid_generate() -> VelvetValue {
    VelvetValue::new("string", &uuid::Uuid::new_v4().to_string())
}

pub fn time_now() -> VelvetValue {
    VelvetValue::new("string", &Utc::now().to_rfc3339())
}

pub fn async_http_get(url: &str) -> VelvetValue {
    VelvetValue::new("string", &format!("Async HTTP GET: {}", url))
}
