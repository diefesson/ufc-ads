#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Client {
    pub priority: bool,
    pub check_in: u32,
}
