use std::fmt::Display;

use super::Client;

pub struct Counter {
    current_client: Option<Client>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            current_client: None,
        }
    }

    pub fn in_use(&self) -> bool {
        self.current_client.is_some()
    }

    pub fn serve(&mut self, client: Client) {
        assert!(!self.in_use(), "counter already in use");
        self.current_client = Some(client)
    }

    pub fn finalize_service(&mut self) {
        assert!(self.in_use(), "counter is not in use");
        self.current_client = None;
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(client) = &self.current_client {
            write!(f, "{}", client)
        } else {
            write!(f, "Free")
        }
    }
}
