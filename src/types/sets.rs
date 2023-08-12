use std::{collections::HashSet, fmt, hash::{Hash, Hasher}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Setable {
    String(String),
    Integer(i64),
    Float(EqFloat),
    Boolean(bool),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EqFloat(pub f64);

impl Eq for EqFloat {}

impl Hash for EqFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl From<f64> for EqFloat {
    fn from(value: f64) -> Self {
        Self(value)
    }
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
        if self.data.is_empty() {
            return 0;
        }

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
            Setable::Float(ef) => write!(f, "{}", ef),
            Setable::Empty => write!(f, "()"),
        }
    }
}

impl fmt::Display for EqFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
