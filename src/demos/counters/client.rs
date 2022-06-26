use std::{error, fmt::Display, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PriorityType {
    NORMAL,
    PRIORITY,
}

impl FromStr for PriorityType {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "normal" => Ok(Self::NORMAL),
            "priority" => Ok(Self::PRIORITY),
            _ => Err("invalid string".into()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ServiceType {
    PARTICULAR,
    BUSINESS,
}

impl FromStr for ServiceType {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "particular" => Ok(Self::PARTICULAR),
            "business" => Ok(Self::BUSINESS),
            _ => Err("invalid string".into()),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Client {
    pub check_in: u32,
    pub priority: PriorityType,
    pub service_type: ServiceType,
}

impl Ord for Client {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.check_in.cmp(&other.check_in).reverse()
    }
}

impl PartialOrd for Client {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} - {:?} - {}",
            self.priority, self.service_type, self.check_in
        )
    }
}
