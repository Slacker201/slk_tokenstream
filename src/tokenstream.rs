use crate::bookmark::Mark;

/// A generic TokenStream struct that manages a stream of tokens with cursor and bookmark functionality.
///
/// # Examples
///
/// ``` rust
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
    /// Creates a new TokenStream from a vector of tokens. Sets cursor to 0.
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.peek(), Some(&1));
    /// assert_eq!(token_stream.peek_offset(1), Some(&2));
    /// assert_eq!(token_stream.peek_offset(2), Some(&3));
    /// ```
    pub fn new(data: Vec<T>) -> Self {
        TokenStream { data, cursor: 0 }
    }
    /// Advances the cursor and returns the next token if available, otherwise returns None.
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.consume(), Some(&1));
    /// assert_eq!(token_stream.consume(), Some(&2));
    /// assert_eq!(token_stream.consume(), Some(&3));
    /// assert_eq!(token_stream.consume(), None);
    /// ```
    pub fn consume(&mut self) -> Option<&T> {
        self.data.get(self.cursor).inspect(|_| self.cursor += 1)
    }
    /// Peeks at the token at the current cursor position without advancing the cursor.
    /// 
    /// # Examples
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.peek(), Some(&1));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.peek_offset(0)
    }
    /// Peeks at the current cursor position plus an offset without advancing the cursor.
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.peek(), Some(&1));
    /// assert_eq!(token_stream.peek_offset(1), Some(&2));
    /// assert_eq!(token_stream.peek_offset(2), Some(&3));
    /// ```
    pub fn peek_offset(&self, offset: usize) -> Option<&T> {
        self.data.get(self.cursor + offset)
    }
    /// Moves the cursor back by one position, saturating at zero.
    /// 
    /// # Examples
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.consume(), Some(&1));
    /// token_stream.rewind();
    /// assert_eq!(token_stream.consume(), Some(&1));
    /// ```
    pub fn rewind(&mut self) {
        self.rewind_offset(1);
    }
    /// Rewinds the cursor a specified amount of times, saturating at 0.
    /// 
    /// # Examples
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.consume(), Some(&1));
    /// assert_eq!(token_stream.consume(), Some(&2));
    /// assert_eq!(token_stream.consume(), Some(&3));
    /// assert_eq!(token_stream.consume(), None);
    /// token_stream.rewind_offset(2);
    /// assert_eq!(token_stream.consume(), Some(&2));
    /// ```
    pub fn rewind_offset(&mut self, offset: usize) {
        self.cursor = self.cursor.saturating_sub(offset);
    }
    /// Returns a mark to the current cursor position.
    /// 
    /// # Examples
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// let mark = token_stream.mark();
    /// token_stream.advance(5);
    /// assert_eq!(token_stream.peek(), None);
    /// token_stream.reset(&mark);
    /// assert_eq!(token_stream.peek(), Some(&1));
    /// ```
    pub fn mark(&self) -> Mark {
        Mark::new(self.cursor)
    }
    /// Moves the cursor to the position of a previously registered bookmark by handle and returns the previous position
    /// 
    /// # Examples
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// let mark = token_stream.mark();
    /// token_stream.advance(3);
    /// assert_eq!(token_stream.peek(), None);
    /// assert_eq!(token_stream.reset(&mark), 3);
    /// assert_eq!(token_stream.peek(), Some(&1));
    /// ```
    pub fn reset(&mut self, bookmark: &Mark) -> usize {
        let old = self.cursor;
        self.cursor = bookmark.idx();
        old
    }
    /// Returns the amount of tokens remaining, including the current token
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.tokens_remaining(), 3);
    /// assert_eq!(token_stream.consume(), Some(&1));
    /// assert_eq!(token_stream.tokens_remaining(), 2);
    /// assert_eq!(token_stream.consume(), Some(&2));
    /// assert_eq!(token_stream.tokens_remaining(), 1);
    /// ```
    pub fn tokens_remaining(&self) -> usize {
        self.data.len().saturating_sub(self.cursor)
    }
    /// Returns if the current token is the end of file
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// assert!(!token_stream.is_eof());
    /// assert_eq!(token_stream.consume(), Some(&1));
    /// assert_eq!(token_stream.consume(), Some(&2));
    /// assert_eq!(token_stream.consume(), Some(&3));
    /// assert!(token_stream.is_eof());
    /// ```
    pub fn is_eof(&self) -> bool {
        self.peek().is_none()
    }
    /// Returns a slice which starts on the earliest mark and ends on the latest.
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// let mark_1 = token_stream.mark();
    /// token_stream.advance(3);
    /// let mark_2 = token_stream.mark();
    /// 
    /// assert_eq!(token_stream.slice_from_marks(&mark_1, &mark_2), &[1, 2, 3]);
    /// assert_eq!(token_stream.slice_from_marks(&mark_2, &mark_1), &[1, 2, 3]);
    /// ```
    pub fn slice_from_marks(&self, mark_1: &Mark, mark_2: &Mark) -> &[T] {
        let mut idx_1 = mark_1.idx();
        let mut idx_2 = mark_2.idx();
        if idx_1 >= idx_2 {
            std::mem::swap(&mut idx_1, &mut idx_2);
        }
        &self.data[idx_1..idx_2]
    }
    /// Advances the cursor by specified amount
    ///
    /// Cursor is clamped to the length of the data
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// token_stream.advance(2);
    /// assert_eq!(token_stream.peek(), Some(&3));
    /// ```
    pub fn advance(&mut self, offset: usize) {
        self.cursor = self.data.len().min(self.cursor + offset);
    }
    /// Returns the next item if it exists and the closure returns true
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.peek_if(|token| *token == 1), Some(&1));
    /// assert_eq!(token_stream.peek_if(|token| *token == 2), None);
    /// ```
    pub fn peek_if<F: Fn(&T) -> bool>(&self, f: F) -> Option<&T> {
        match self.peek() {
            Some(v) if f(v) => Some(v),
            _ => None,
        }
    }
    /// Returns the next item and advances the cursor if the item exists and the closure returns true
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.expect(|token| *token == 1), Some(&1));
    /// assert_eq!(token_stream.expect(|token| *token == 2), Some(&2));
    /// ```
    pub fn expect<F: Fn(&T) -> bool>(&mut self, f: F) -> Option<&T> {
        let ok = match self.peek() {
            Some(v) if f(v) => true,
            _ => false,
        };
        if ok { self.consume() } else { None }
    }
    /// Returns a slice of items starting from the cursor and ending when the closure returns false. The cursor remains on the first item failing the test
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.consume_while(|token| *token < 3), &[1, 2]);
    /// assert_eq!(token_stream.peek(), Some(&3));
    /// ```
    pub fn consume_while<F: Fn(&T) -> bool>(&mut self, f: F) -> &[T] {
        let m1 = self.mark();
        while self.expect(&f).is_some() {}
        let m2 = self.mark();
        let slice = self.slice_from_marks(&m1, &m2);
        slice
    }
    /// Returns a slice of items starting from the cursor and ending when the closure returns false. The cursor remains in the original position
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.peek_while(|token| *token < 3), &[1, 2]);
    /// assert_eq!(token_stream.peek(), Some(&1));
    /// ```
    pub fn peek_while<F: Fn(&T) -> bool>(&mut self, f: F) -> &[T] {
        let m1 = self.mark();
        while self.expect(&f).is_some() {}
        let m2 = self.mark();
        self.reset(&m1);
        let slice = self.slice_from_marks(&m1, &m2);
        slice
    }
    /// Advances the cursor 1 step
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// token_stream.skip();
    /// assert_eq!(token_stream.peek(), Some(&2));
    /// token_stream.skip();
    /// assert_eq!(token_stream.peek(), Some(&3));
    /// ```
    pub fn skip(&mut self) {
        self.advance(1);
    }
    /// Advances the cursor one step if the closure returns true
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// token_stream.skip_if(|token| *token == 1);
    /// assert_eq!(token_stream.peek(), Some(&2));
    /// token_stream.skip_if(|token| *token == 1);
    /// assert_eq!(token_stream.peek(), Some(&2));
    /// ```
    pub fn skip_if<F: Fn(&T) -> bool>(&mut self, f: F) {
        match self.peek_if(f) {
            Some(_) => self.skip(),
            None => {}
        }
    }

    /// Returns the current position of the cursor
    /// 
    /// # Examples
    /// 
    /// ``` rust
    /// use tokenstream::tokenstream::TokenStream;
    /// let tokens = vec![1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// assert_eq!(token_stream.position(), 0);
    /// assert_eq!(token_stream.consume(), Some(&1));
    /// assert_eq!(token_stream.position(), 1);
    /// assert_eq!(token_stream.consume(), Some(&2));
    /// assert_eq!(token_stream.position(), 2);
    /// assert_eq!(token_stream.consume(), Some(&3));
    /// assert_eq!(token_stream.position(), 3);
    /// ```
    pub fn position(&self) -> usize {
        self.cursor
    }
}
