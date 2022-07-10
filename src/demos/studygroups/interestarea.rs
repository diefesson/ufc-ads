use std::{
    error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterestArea {
    Eda,
    Fup,
    Paa,
    Bd,
    C1,
}

impl InterestArea {
    pub const VALUES: [InterestArea; 5] = [Self::Eda, Self::Fup, Self::Paa, Self::Bd, Self::C1];
}

impl FromStr for InterestArea {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(index) = s.parse::<usize>() {
            if let Some(interest_area) = InterestArea::VALUES.get(index) {
                Ok(*interest_area)
            } else {
                Err("invalid index".into())
            }
        } else {
            Err("invalid string".into())
        }
    }
}

impl Display for InterestArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Eda => write!(f, "Advanced Data Structures"),
            Self::Fup => write!(f, "Programming Fundamentals"),
            Self::Paa => write!(f, "Project and Algorith Analysis"),
            Self::Bd => write!(f, "Databases"),
            Self::C1 => write!(f, "Calculus 1"),
        }
    }
}
