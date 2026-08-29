use crate::TokenStream;

impl<'a, T> TokenStream<'a, T> {
    /// Sets the cursor to `new_value`, clamping it within the valid range
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slk_tokenstream::TokenStream;
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// token_stream.set_cursor(1);
    /// assert_eq!(token_stream.position(), 1);
    ///
    ///
    /// token_stream.set_cursor(100);
    /// assert_eq!(token_stream.position(), 3);
    /// ```
    pub fn set_cursor(&mut self, new_value: usize) {
        self.cursor = new_value.min(self.data.len());
    }
    /// Returns if the current token is the end of file
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// assert!(!token_stream.is_eof());
    /// assert_eq!(token_stream.consume(), Some(&1));
    /// assert_eq!(token_stream.consume(), Some(&2));
    /// assert_eq!(token_stream.consume(), Some(&3));
    /// assert!(token_stream.is_eof());
    /// ```
    pub fn is_eof(&self) -> bool {
        self.peek().is_none()
    }
    /// Returns the amount of tokens remaining, including the current token
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// assert_eq!(token_stream.tokens_remaining(), 3);
    /// assert_eq!(token_stream.consume(), Some(&1));
    /// assert_eq!(token_stream.tokens_remaining(), 2);
    /// assert_eq!(token_stream.consume(), Some(&2));
    /// assert_eq!(token_stream.tokens_remaining(), 1);
    /// ```
    pub fn tokens_remaining(&self) -> usize {
        self.data.len().saturating_sub(self.cursor)
    }
    /// Moves the cursor back by one position, saturating at zero.
    ///
    /// # Examples
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
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
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
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

    /// Returns the current position of the cursor
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
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

    /// An alias of position.
    /// Returns the current position of the cursor
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// assert_eq!(token_stream.cursor(), 0);
    /// assert_eq!(token_stream.consume(), Some(&1));
    /// assert_eq!(token_stream.cursor(), 1);
    /// assert_eq!(token_stream.consume(), Some(&2));
    /// assert_eq!(token_stream.cursor(), 2);
    /// assert_eq!(token_stream.consume(), Some(&3));
    /// assert_eq!(token_stream.cursor(), 3);
    /// ```
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Sets the cursor to `new_value` without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `new_value` is a valid cursor position
    /// (i.e. `new_value <= self.data.len()`). Using an out-of-bounds value
    /// may lead to panics or undefined behavior in subsequent operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slk_tokenstream::TokenStream;
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    ///
    /// unsafe { token_stream.set_cursor_unchecked(1); }
    /// assert_eq!(token_stream.position(), 1);
    /// ```
    pub unsafe fn set_cursor_unchecked(&mut self, new_value: usize) {
        self.cursor = new_value;
    }
}
