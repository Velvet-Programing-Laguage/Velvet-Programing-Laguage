use crate::types::VelvetValue;

pub fn json_parse(json: &str) -> VelvetValue {
    VelvetValue::new("string", format!("Parsed JSON: {}", json))
}

pub fn ai_predict(data: &str) -> VelvetValue {
    VelvetValue::new("string", format!("AI prediction: {}", data))
}

pub fn uuid_generate() -> VelvetValue {
    VelvetValue::new("string", "uuid-1234-5678")
}
