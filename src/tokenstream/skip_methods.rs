use crate::TokenStream;

impl<'a, T> TokenStream<'a, T> {
    /// Advances the cursor by specified amount
    ///
    /// Cursor is clamped to the length of the data
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// token_stream.advance(2);
    /// assert_eq!(token_stream.peek(), Some(&3));
    /// ```
    pub fn advance(&mut self, offset: usize) {
        self.cursor = self.data.len().min(self.cursor.saturating_add(offset));
    }
    /// Advances the cursor 1 step
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
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
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
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

    /// Advances the cursor until the closure returns false
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// token_stream.skip_while(|token| *token < 3);
    /// assert_eq!(token_stream.peek(), Some(&3));
    /// ```
    pub fn skip_while<F: Fn(&T) -> bool>(&mut self, f: F) {
        while self.peek_if(&f).is_some() {
            self.advance(1);
        }
    }

    /// Advances the cursor if the next item exists and the closure returns true
    /// Returns `Err(Some(&T))` if the next item exists and the closure returns false
    /// Returns `Err(None)` if the item does not exist
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slk_tokenstream::TokenStream;
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// assert_eq!(token_stream.skip_if_else_err(|c| *c == 1), Ok(()));
    /// assert_eq!(token_stream.skip_if_else_err(|c| *c == 1), Err(Some(&2)));
    /// assert_eq!(token_stream.skip_if_else_err(|c| *c == 1), Err(Some(&2)));
    /// ```
    pub fn skip_if_else_err<F: Fn(&T) -> bool>(&mut self, f: F) -> Result<(), Option<&T>> {
        let ok = self.peek().and_then(|c| Some(f(c))).unwrap_or(false);

        if ok {
            self.skip();
            Ok(())
        } else {
            match self.peek() {
                Some(c) => Err(Some(c)),
                None => Err(None),
            }
        }
    }
}
