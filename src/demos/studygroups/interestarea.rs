use std::fmt::{self, Display, Formatter};

pub enum InterestArea {
    EDA,
    FUP,
    PAA,
    BD,
    C1,
}

impl Display for InterestArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::EDA => write!(f, "Advanced Data Structures"),
            Self::FUP => write!(f, "Programming Fundamentals"),
            Self::PAA => write!(f, "Project and Algorith Analysis"),
            Self::BD => write!(f, "Databases"),
            Self::C1 => write!(f, "Calculus 1"),
        }
    }
}
