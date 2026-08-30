use crate::Mark;

#[derive(Debug, Clone, Copy, Hash)]
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
    pub fn to_usize(&self) -> (usize, usize) {
        (self.start.position(), self.end.position())
    }
}
