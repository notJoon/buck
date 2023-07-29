use std::fmt;

use super::types::BuckTypes;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BuckList {
    pub data: Vec<BuckTypes>,
}

impl BuckList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn b_push(&mut self, key: &str, value: BuckTypes) {
        unimplemented!("b_push")
    }

    pub fn b_pop(&mut self, key: &str) -> Option<BuckTypes> {
        unimplemented!("b_pop")
    }
}

impl fmt::Display for BuckList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for (i, item) in self.data.iter().enumerate() {
            result.push_str(&format!("{}: {}\n", i, item));
        }
        write!(f, "{}", result)
    }
}