use crate::{Mark, TokenStream};

impl<'a, T> TokenStream<'a, T> {
    /// Advances the cursor and returns `Ok(&T)` if the next item exists and the closure returns true
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
    /// assert_eq!(token_stream.consume_if_else_err(|c| *c == 1), Ok(&1));
    /// assert_eq!(token_stream.consume_if_else_err(|c| *c == 1), Err(Some(&2)));
    /// assert_eq!(token_stream.consume_if_else_err(|c| *c == 1), Err(Some(&2)));
    /// ```
    pub fn consume_if_else_err<F: Fn(&T) -> bool>(&mut self, f: F) -> Result<&T, Option<&T>> {
        let ok = self.peek().and_then(|c| Some(f(c))).unwrap_or(false);

        if ok {
            self.consume().ok_or(None)
        } else {
            match self.peek() {
                Some(c) => Err(Some(c)),
                None => Err(None),
            }
        }
    }
    /// Returns a slice of items starting from the cursor and ending when the closure returns false. The cursor remains in the original position
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// assert_eq!(token_stream.peek_while(|token| *token < 3), &[1, 2]);
    /// assert_eq!(token_stream.peek(), Some(&1));
    /// ```
    pub fn peek_while<F: Fn(&T) -> bool>(&self, f: F) -> &[T] {
        let len = self.data[self.cursor..]
            .iter()
            .take_while(|item| f(item))
            .count();
        &self.data[self.cursor..self.cursor + len]
    }
    /// Returns a slice of items starting from the cursor and ending when the closure returns false. The cursor remains on the first item failing the test
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// assert_eq!(token_stream.consume_while(|token| *token < 3), &[1, 2]);
    /// assert_eq!(token_stream.peek(), Some(&3));
    /// ```
    pub fn consume_while<F: Fn(&T) -> bool>(&mut self, f: F) -> &[T] {
        let m1 = self.mark();
        while self.consume_if(&f).is_some() {}
        let m2 = self.mark();
        let slice = self.slice_from_marks(m1, m2);
        slice
    }
    /// Returns the next item and advances the cursor if the item exists and the closure returns true
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// assert_eq!(token_stream.consume_if(|token| *token == 1), Some(&1));
    /// assert_eq!(token_stream.consume_if(|token| *token == 2), Some(&2));
    /// ```
    pub fn consume_if<F: Fn(&T) -> bool>(&mut self, f: F) -> Option<&T> {
        let ok = match self.peek() {
            Some(v) if f(v) => true,
            _ => false,
        };
        if ok { self.consume() } else { None }
    }
    /// Advances the cursor and returns the next token if available, otherwise returns None.
    ///
    /// # Examples
    ///
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
    /// ```
    pub fn consume(&mut self) -> Option<&T> {
        self.data.get(self.cursor).inspect(|_| self.cursor += 1)
    }

    /// Returns the item at the cursor and advances the cursor if the function evaluates to true,
    /// otherwise rewinds the cursor to the provided mark
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// let mark = token_stream.mark();
    /// assert_eq!(mark.position(), 0);
    ///
    /// assert_eq!(token_stream.expect(|token| *token == 1, mark), Some(&1));
    /// assert_eq!(token_stream.position(), 1);
    /// assert_eq!(token_stream.expect(|token| *token == 3, mark), None);
    /// assert_eq!(token_stream.position(), 0);
    /// ```
    /// ```
    pub fn expect<F: Fn(&T) -> bool>(&mut self, f: F, mark: Mark) -> Option<&T> {
        let t = if let Some(t) = self.data.get(self.cursor) {
            t
        } else {
            self.reset(mark);
            return None;
        };
        if f(t) {
            self.skip();
            return Some(t);
        } else {
            self.reset(mark);
            return None;
        }
    }

    /// Advances the cursor by the length of the slice and returns it if it matches
    /// 
    /// # Examples
    /// ```rust
    /// use slk_tokenstream::TokenStream;
    /// let tokens = &[1, 2, 3, 4];
    /// let mut token_stream = TokenStream::new(tokens);
    /// let test_slice: &[i32] = &[1, 2, 3];
    /// 
    /// 
    /// assert_eq!(token_stream.consume_if_matches_slice(test_slice), Some(test_slice));
    /// assert_eq!(token_stream.peek(), Some(&4));
    /// assert_eq!(token_stream.consume_if_matches_slice(test_slice), None);
    /// assert_eq!(token_stream.peek(), Some(&4));
    /// ```
    pub fn consume_if_matches_slice(&mut self, slice: &[T]) -> Option<&[T]>
    where
        T: PartialEq,
    {
        let start = self.mark();
        self.advance(slice.len());
        let end = self.mark();
        
        if self.slice_from_marks(start, end) == slice {
            self.reset(end);
            Some(self.slice_from_marks(start, end))
        } else {
            self.reset(start);
            None
        }
    }
}
