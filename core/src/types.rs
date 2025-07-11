#[derive(Debug)]
pub struct VelvetValue {
    pub kind: String,
    pub value: String,
}

impl VelvetValue {
    pub fn new(kind: &str, value: &str) -> Self {
        VelvetValue {
            kind: kind.to_string(),
            value: value.to_string(),
        }
    }
}
