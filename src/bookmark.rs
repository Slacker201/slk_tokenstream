#[derive(Debug)]
pub struct Mark {
    idx: usize,
}

impl Mark {
    /// Creates a new mark with the provided id
    pub fn new(idx: usize) -> Self {
        Self { idx }
    }
    /// Returns the stored index
    pub(crate) fn idx(&self) -> usize {
        self.idx
    }
}
