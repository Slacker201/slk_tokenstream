use std::{hash::Hash, sync::Arc};

#[derive(Debug)]
pub struct Mark {
    pub inner: Inner,
}

impl Mark {
    /// Creates a new mark with the provided id
    pub fn new(id: u64) -> Self {
        Self {
            inner: Inner(id, Arc::new(())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Inner(u64, Arc<()>);

impl PartialEq for Inner {
    /// Compares only the id
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for Inner {
    /// Hashes only the id
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Eq for Inner {}

impl Inner {
    /// Get the strong count
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.1)
    }
    /// Get the id
    pub fn id(&self) -> u64 {
        self.0
    }
}
