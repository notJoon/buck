use std::fmt;

use super::{types::BuckTypes, errors::BuckTypeError};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BuckList {
    pub data: Vec<BuckTypes>,
}

impl BuckList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, value: BuckTypes) {
        self.data.insert(0, value);
    }

    pub fn pop(&mut self, pos: &str) -> Result<Option<BuckTypes>, BuckTypeError> {
        if self.data.is_empty() {
            return Err(BuckTypeError::ListIsEmpty);
        }

        match pos {
            "head" => {
                let pop_head = self.data.remove(0);
                Ok(Some(pop_head))
            },
            "tail" => {
                let pop_tail = self.data.pop();
                Ok(pop_tail)
            },
            _ => Err(BuckTypeError::UnknownCommand(pos.to_owned())),
        }
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