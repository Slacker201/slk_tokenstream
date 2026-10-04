use crate::{Mark, TokenStream};

impl<'a, T> TokenStream<'a, T> {
    /// Returns `Ok(&T)` if the next item exists and the closure returns true
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
    /// assert_eq!(token_stream.peek_if_else_err(|c| *c == 1), Ok(&1));
    /// assert_eq!(token_stream.peek_if_else_err(|c| *c == 1), Ok(&1));
    /// token_stream.skip();
    /// assert_eq!(token_stream.peek_if_else_err(|c| *c == 1), Err(Some(&2)));
    /// ```
    pub fn peek_if_else_err<F: Fn(&T) -> bool>(&self, f: F) -> Result<&T, Option<&T>> {
        let ok = self.peek().and_then(|c| Some(f(c))).unwrap_or(false);

        if ok {
            self.peek().ok_or(None)
        } else {
            match self.peek() {
                Some(c) => Err(Some(c)),
                None => Err(None),
            }
        }
    }
    /// Returns the next item if it exists and the closure returns true
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// assert_eq!(token_stream.peek_if(|token| *token == 1), Some(&1));
    /// assert_eq!(token_stream.peek_if(|token| *token == 2), None);
    /// ```
    pub fn peek_if<F: Fn(&T) -> bool>(&self, f: F) -> Option<&T> {
        match self.peek() {
            Some(v) if f(v) => Some(v),
            _ => None,
        }
    }
    /// Peeks at the token at the current cursor position without advancing the cursor.
    ///
    /// # Examples
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let token_stream = TokenStream::new(tokens);
    ///
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
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let token_stream = TokenStream::new(tokens);
    ///
    /// assert_eq!(token_stream.peek(), Some(&1));
    /// assert_eq!(token_stream.peek_offset(1), Some(&2));
    /// assert_eq!(token_stream.peek_offset(2), Some(&3));
    /// ```
    pub fn peek_offset(&self, offset: usize) -> Option<&T> {
        self.data.get(self.cursor.saturating_add(offset))
    }

    /// Returns the slice if it matches
    /// 
    /// # Examples
    /// ```rust
    /// use slk_tokenstream::TokenStream;
    /// let tokens = &[1, 2, 3, 4];
    /// let mut token_stream = TokenStream::new(tokens);
    /// let test_slice: &[i32] = &[1, 2, 3];
    /// 
    /// 
    /// assert_eq!(token_stream.peek_if_matches_slice(test_slice), Some(test_slice));
    ///
    /// let test_slice = &[2, 3, 4];
    /// 
    /// assert_eq!(token_stream.peek_if_matches_slice(test_slice), None);
    /// ```
    pub fn peek_if_matches_slice(&self, slice: &[T]) -> Option<&[T]>
    where
        T: PartialEq,
    {
        let start = self.mark();

        let end = Mark::new(start.position() + slice.len());

        let our_slice = self.slice_from_marks(start, end);
        if our_slice == slice {
            Some(our_slice)
        } else {
            None
        }
    }
}
