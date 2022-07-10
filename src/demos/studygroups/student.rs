use std::fmt::Display;

use super::InterestArea;

pub struct Student {
    pub id: usize,
    pub name: String,
    pub interest_areas: Vec<InterestArea>,
}

impl Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {} : ", self.id, self.name)?;
        let mut interest_areas = self.interest_areas.iter();
        if let Some(interest_area) = interest_areas.next() {
            write!(f, "{}", interest_area)?;
            for interest_area in interest_areas {
                write!(f, ", {}", interest_area)?;
            }
        }
        Ok(())
    }
}
