use super::Client;

pub struct Counter {
    current_client: Option<Client>,
    priority: bool,
    open: bool,
}

impl Counter {
    fn new(priority: bool) -> Self {
        Self {
            current_client: None,
            priority,
            open: true,
        }
    }

    fn in_use(&self) -> bool {
        self.current_client.is_some()
    }

    fn serve(&mut self, client: Client) {
        assert!(self.is_open());
        assert!(!self.in_use());
        self.current_client = Some(client)
    }

    fn is_priority(&self) -> bool {
        self.priority
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn open(&mut self) {
        self.open = true
    }

    fn close(&mut self) {
        assert!(!self.in_use());
        self.open = false
    }
}
