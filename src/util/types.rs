use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct TimeStamped<T> {
    pub timestamp: f64,
    pub value:     T,
}

impl<T> TimeStamped<T> {
    pub fn update(&mut self, timestamp: f64, new_value: T) where T: PartialEq {
        if self.value != new_value {
            self.value = new_value;
            self.timestamp = timestamp;
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Embeddable<T> {
    Embedded(T),
    File(String),
}