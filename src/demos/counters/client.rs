use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Client {
    pub priority: bool,
    pub check_in: u32,
}

impl Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Priority: {}, Check in: {} ",
            self.priority, self.check_in
        )
    }
}
