#[derive(Debug)]
pub struct Mark {
    position: usize,
}

impl Mark {
    /// Creates a new mark with the position
    pub(crate) fn new(position: usize) -> Self {
        Self { position }
    }
    /// Returns the position
    pub fn position(&self) -> usize {
        self.position
    }
}
