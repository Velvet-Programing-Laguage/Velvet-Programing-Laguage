use crate::types::VelvetValue;
use chrono::Utc;
use tokio::time::sleep;
use std::time::Duration;
use uuid::Uuid;

/// Placeholder: nie parsuje prawdziwego JSON-a! Skorzystaj z serde_json dla produkcyjnego kodu.
pub fn json_parse(json: &str) -> VelvetValue {
    VelvetValue::new("string", &format!("Parsed JSON: {}", json))
}

/// Placeholder: nie zamienia na prawdziwy JSON! Skorzystaj z serde_json dla produkcyjnego kodu.
pub fn json_stringify(data: &str) -> VelvetValue {
    VelvetValue::new("string", &format!("Stringified JSON: {}", data))
}

/// Placeholder: nie wykonuje prawdziwego AI. W produkcji podłącz model ML lub API.
pub fn ai_predict(data: &str) -> VelvetValue {
    VelvetValue::new("string", &format!("AI prediction: {}", data))
}

/// Generuje UUID v4.
pub fn uuid_generate() -> VelvetValue {
    VelvetValue::new("string", &Uuid::new_v4().to_string())
}

/// Zwraca bieżący czas (UTC) w formacie RFC3339.
pub fn time_now() -> VelvetValue {
    VelvetValue::new("string", &Utc::now().to_rfc3339())
}

/// Placeholder dla żądania HTTP GET. W produkcji użyj np. biblioteki reqwest.
pub async fn async_http_get(url: &str) -> VelvetValue {
    sleep(Duration::from_millis(100)).await;
    VelvetValue::new("string", &format!("Async HTTP GET: {}", url))
}

/// Opóźnia wykonanie o podaną liczbę milisekund.
pub async fn async_delay(ms: u64) -> VelvetValue {
    sleep(Duration::from_millis(ms)).await;
    VelvetValue::new("string", &format!("Delayed for {}ms", ms))
}
