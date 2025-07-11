use crate::types::VelvetValue;

pub fn json_parse(json: &str) -> VelvetValue {
    VelvetValue::new("string", format!("Parsed JSON: {}", json))
}

pub fn json_stringify(data: &str) -> VelvetValue {
    VelvetValue::new("string", format!("Stringified JSON: {}", data))
}

pub fn ai_predict(data: &str) -> VelvetValue {
    VelvetValue::new("string", format!("AI prediction: {}", data))
}

pub fn uuid_generate() -> VelvetValue {
    VelvetValue::new("string", uuid::Uuid::new_v4().to_string())
}

pub fn time_now() -> VelvetValue {
    VelvetValue::new("string", chrono::Utc::now().to_rfc3339())
}
