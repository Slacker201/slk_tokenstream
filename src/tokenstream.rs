use crate::bookmark::Mark;

/// A generic TokenStream struct that manages a stream of tokens with cursor and bookmark functionality.
///
/// # Examples
///
/// ```
/// use tokenstream::tokenstream::TokenStream;
/// use tokenstream::bookmark::Mark;
/// let tokens = vec![1, 2, 3];
/// let mut token_stream = TokenStream::new(tokens);
///
/// assert_eq!(token_stream.consume(), Some(&1));
/// assert_eq!(token_stream.peek(), Some(&2));
/// assert_eq!(token_stream.tokens_remaining(), 2);
/// ```
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
    pub fn mark(&self) -> Mark {
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
    /// Returns a slice which starts on mark_1 and ends on mark_2
    pub fn slice_from_marks(&self, mark_1: &Mark, mark_2: &Mark) -> &[T] {
        let idx_1 = mark_1.idx();
        let idx_2 = mark_2.idx();
        &self.data[idx_1..idx_2]
    }
    /// Advances the cursor by specified amount
    ///
    /// Cursor is clamped to the length of the data
    pub fn advance(&mut self, offset: usize) {
        self.cursor = self.data.len().min(self.cursor + offset);
    }
    /// Returns the next item if it exists and the closure returns true
    pub fn peek_if<F: Fn(&T) -> bool>(&self, f: F) -> Option<&T> {
        match self.peek() {
            Some(v) if f(v) => Some(v),
            _ => None,
        }
    }
    /// Returns the next item and advances the cursor if the item exists and the closure returns true
    pub fn expect<F: Fn(&T) -> bool>(&mut self, f: F) -> Option<&T> {
        let ok = match self.peek() {
            Some(v) if f(v) => true,
            _ => false,
        };
        if ok { self.consume() } else { None }
    }
    /// Returns a slice of items starting from the cursor and ending when the closure returns false
    ///
    /// Cursor remains on the failed item
    pub fn consume_while<F: Fn(&T) -> bool>(&mut self, f: F) -> &[T] {
        let m1 = self.mark();
        while self.expect(&f).is_some() {}
        let m2 = self.mark();
        let slice = self.slice_from_marks(&m1, &m2);
        slice
    }
    /// Returns a slice of items starting from the cursor and ending when the closure returns false
    ///
    /// Cursor resets to its original position
    pub fn peek_while<F: Fn(&T) -> bool>(&mut self, f: F) -> &[T] {
        let m1 = self.mark();
        while self.expect(&f).is_some() {}
        let m2 = self.mark();
        self.reset(&m1);
        let slice = self.slice_from_marks(&m1, &m2);
        slice
    }
    /// Advances the cursor 1 step
    pub fn skip(&mut self) {
        self.advance(1);
    }
    /// Advances the cursor one step if the closure returns true
    pub fn skip_if<F: Fn(&T) -> bool>(&mut self, f: F) {
        match self.peek_if(f) {
            Some(_) => self.skip(),
            None => {}
        }
    }
}
