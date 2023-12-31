use std::fmt;

use super::{errors::BuckTypeError, types::BuckTypes};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BuckList {
    pub data: Vec<BuckTypes>,
}

impl BuckList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, value: BuckTypes) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Result<Option<BuckTypes>, BuckTypeError> {
        if self.data.is_empty() {
            return Err(BuckTypeError::ListIsEmpty);
        }

        Ok(self.data.pop())
    }

    pub fn len(&self) -> usize {
        self.data.len()
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
