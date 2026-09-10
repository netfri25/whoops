#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Response {
    #[default]
    Ignored,
    Applied,
    Consumed,
}

impl Response {
    pub fn is_applied(self) -> bool {
        self >= Self::Applied
    }

    pub fn is_consumed(self) -> bool {
        self >= Self::Consumed
    }
}
