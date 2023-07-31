use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Setable {
    String(String),
    Integer(i64),
    Boolean(bool),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BuckSets {
    pub data: HashSet<Setable>,
}

impl BuckSets {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, value: &[Setable]) {
        for item in value {
            self.data.insert(item.clone());
        }
    }

    pub fn remove(&mut self, value: &[Setable]) {
        for item in value {
            self.data.remove(item);
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn intersection(&self, other: &[BuckSets]) -> Self {
        let mut result = BuckSets::new();

        for item in &self.data {
            let mut is_in_all = true;
            for set in other {
                if !set.data.contains(item) {
                    is_in_all = false;
                    break;
                }
            }

            if is_in_all {
                result.data.insert(item.clone());
            }
        }

        result
    }

    pub fn is_member(&self, value: &Setable) -> bool {
        self.data.contains(value)
    }
}

impl fmt::Display for Setable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Setable::String(s) => write!(f, "{}", s),
            Setable::Integer(i) => write!(f, "{}", i),
            Setable::Boolean(b) => write!(f, "{}", b),
        }
    }
}
