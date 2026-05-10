

#[derive(Debug)]
pub struct Mark {
    pub id: u64,
}


impl Mark {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}