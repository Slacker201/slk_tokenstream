use crate::bookmark::Mark;

/// A generic TokenStream struct that manages a stream of tokens with cursor and bookmark functionality.
#[derive(Debug)]
pub struct TokenStream<T> {
    data: Vec<T>,
    cursor: usize,
}

impl<T> TokenStream<T> {
    /// Creates a new TokenStream from a vector of tokens. Sets cursor to 0 and initializes an empty bookmark map.
    pub fn new(data: Vec<T>) -> Self {
        TokenStream { data, cursor: 0 }
    }
    /// Advances the cursor and returns the next token if available, otherwise returns None.
    pub fn consume(&mut self) -> Option<&T> {
        self.data.get(self.cursor).inspect(|_| self.cursor += 1)
    }
    /// Peeks at the token at the current cursor position plus an optional offset without advancing the cursor.
    pub fn peek(&self) -> Option<&T> {
        self.peek_offset(0)
    }
    pub fn peek_offset(&self, offset: usize) -> Option<&T> {
        self.data.get(self.cursor + offset)
    }
    /// Moves the cursor back by one position, saturating at 0.
    pub fn rewind(&mut self) {
        self.rewind_offset(1);
    }
    /// Rewinds the cursor a specified amount of times, saturating at 0.
    pub fn rewind_offset(&mut self, offset: usize) {
        self.cursor = self.cursor.saturating_sub(offset);
    }
    /// Returns a mark to the current cursor position.
    pub fn mark(&mut self) -> Mark {
        Mark::new(self.cursor)
    }
    /// Moves the cursor to the position of a previously registered bookmark by handle.
    ///
    /// Returns the previous cursor position.
    pub fn reset(&mut self, bookmark: &Mark) -> usize {
        let old = self.cursor;
        self.cursor = bookmark.idx();
        old
    }
    /// Returns the amount of tokens remaining, including the current token
    pub fn tokens_remaining(&self) -> usize {
        self.data.len() - self.cursor
    }
    /// Returns if the current token is the end of file
    pub fn is_eof(&self) -> bool {
        self.peek().is_none()
    }
}