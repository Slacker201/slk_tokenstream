use core::marker::PhantomData;

use crate::Mark;



pub struct TokenstreamSpan {
    start: Mark,
    end: Mark,
}

impl TokenstreamSpan {
    pub fn new(mut start: Mark, mut end: Mark) -> Self {
        if start.position() > end.position() {
            core::mem::swap(&mut start, &mut end);
        }
        Self { start, end }
    }
    pub fn start(&self) -> Mark {
        self.start
    }
    pub fn end(&self) -> Mark {
        self.end
    }
}