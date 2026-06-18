use core::marker::PhantomData;

use crate::Mark;



pub struct TokenstreamSpan<'a> {
    start: Mark<'a>,
    end: Mark<'a>,
    _lifetime: PhantomData<&'a ()>
}

impl<'a> TokenstreamSpan<'a> {
    pub fn new(mut start: Mark<'a>, mut end: Mark<'a>) -> Self {
        if start.position() > end.position() {
            core::mem::swap(&mut start, &mut end);
        }
        Self { start, end, _lifetime: PhantomData }
    }
    pub fn start(&self) -> Mark<'a> {
        self.start
    }
    pub fn end(&self) -> Mark<'a> {
        self.end
    }
}