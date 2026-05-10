

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Mark {
    pub id: u64,
}


impl Mark {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}