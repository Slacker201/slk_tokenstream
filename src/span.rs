use core::marker::PhantomData;

use crate::Mark;



pub struct TokenstreamSpan<'a> {
    start: Mark,
    end: Mark,
    _lifetime: PhantomData<&'a ()>
}

impl<'a> TokenstreamSpan<'a> {
    pub fn new(start: Mark, end: Mark) -> Self {
        Self { start, end, _lifetime: PhantomData }
    }
    pub fn start(&self) -> Mark {
        self.start
    }
    pub fn end(&self) -> Mark {
        self.end
    }
}