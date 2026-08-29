use crate::{Mark, TokenStream, TokenstreamSpan};

impl<'a, T> TokenStream<'a, T> {
    /// Returns a slice from two Marks, ensuring they are ordered correctly
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// let mark_1 = token_stream.mark();
    /// token_stream.advance(3);
    /// let mark_2 = token_stream.mark();
    ///
    ///
    /// assert_eq!(token_stream.slice_from_marks(mark_1, mark_2), &[1, 2, 3]);
    /// ```
    pub fn slice_from_marks(&self, mut start: Mark, mut end: Mark) -> &[T] {
        if start.position() > end.position() {
            core::mem::swap(&mut start, &mut end);
        }
        let idx_1 = start.position();
        let idx_2 = end.position();
        &self.data[idx_1..idx_2]
    }
    /// Returns a slice from a span
    ///
    /// # Examples
    ///
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// let mark_1 = token_stream.mark();
    /// token_stream.advance(3);
    /// let mark_2 = token_stream.mark();
    ///
    /// let span = token_stream.span_from_marks(mark_1, mark_2);
    ///
    /// assert_eq!(token_stream.slice_from_span(span), &[1, 2, 3]);
    /// ```
    pub fn slice_from_span(&self, span: TokenstreamSpan) -> &[T] {
        let idx_1 = span.start().position();
        let idx_2 = span.end().position();
        &self.data[idx_1..idx_2]
    }
    /// Returns a mark to the current cursor position.
    ///
    /// # Examples
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// let mark = token_stream.mark();
    ///
    /// token_stream.advance(5);
    /// assert_eq!(token_stream.peek(), None);
    /// token_stream.reset(mark);
    /// assert_eq!(token_stream.peek(), Some(&1));
    /// ```
    pub fn mark(&self) -> Mark {
        Mark::new(self.cursor)
    }

    /// Moves the cursor to the position of a previously registered bookmark by handle and returns the previous position
    ///
    /// # Examples
    /// ``` rust
    /// use slk_tokenstream::TokenStream;
    ///
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    /// let mark = token_stream.mark();
    ///
    /// token_stream.advance(3);
    /// assert_eq!(token_stream.peek(), None);
    /// assert_eq!(token_stream.reset(mark), 3);
    /// assert_eq!(token_stream.peek(), Some(&1));
    /// ```
    pub fn reset(&mut self, bookmark: Mark) -> usize {
        let old = self.cursor;
        self.cursor = bookmark.position();
        old
    }

    /// Creates a `Span` from two marks
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slk_tokenstream::TokenStream;
    /// let tokens = &[1, 2, 3];
    /// let mut token_stream = TokenStream::new(tokens);
    ///
    /// let m1 = token_stream.mark();
    /// token_stream.consume();
    /// let m2 = token_stream.mark();
    ///
    /// let span = token_stream.span_from_marks(m1, m2);
    ///
    /// assert_eq!(token_stream.slice_from_span(span), &[1]);
    /// ```
    pub fn span_from_marks(&self, start: Mark, end: Mark) -> TokenstreamSpan {
        TokenstreamSpan::new(start, end)
    }
}
