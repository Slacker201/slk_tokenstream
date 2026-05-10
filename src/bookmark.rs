use std::{hash::Hash, sync::{Arc, atomic::{AtomicU16, Ordering}}};



#[derive(Debug)]
pub struct Mark {
    pub inner: Inner,
}


impl Mark {
    pub fn new(id: u64) -> Self {
        Self { inner: Inner(id, Arc::new(AtomicU16::new(1))) }
    }
}

impl Drop for Mark {
    fn drop(&mut self) {
        self.inner.1.fetch_sub(1, Ordering::SeqCst);
    }
}

#[derive(Debug, Clone)]
pub struct Inner(u64, Arc<AtomicU16>);

impl PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for Inner {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Eq for Inner {}

impl Inner {
    pub fn tracker_val(&self) -> u16 {
        self.1.load(Ordering::Relaxed )
    }
    pub fn id(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Inner {
    fn from(value: u64) -> Self {
        Self(value, Arc::new(AtomicU16::new(0)))
    }
}