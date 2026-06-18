use core::marker::PhantomData;

/// A mark struct used to mark positions in a `TokenStream` for backtracking    
/// 
/// # Examples
/// ``` rust
/// use slk_tokenstream::TokenStream;
/// use slk_tokenstream::Mark;
/// 
/// let tokens = &[1, 2, 3];
/// let mut token_stream = TokenStream::new(tokens);
/// let mark: Mark = token_stream.mark();
/// 
/// token_stream.advance(5);
/// assert_eq!(token_stream.peek(), None);
/// token_stream.reset(&mark);
/// assert_eq!(token_stream.peek(), Some(&1));
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Mark {
    position: usize,
}

impl Mark {
    /// Creates a new mark with the position
    pub(crate) fn new(position: usize) -> Self {
        Self { position }
    }
    /// Returns the position
    /// 
    /// # Example
    /// 
    /// ```
    /// use slk_tokenstream::Mark;
    /// use slk_tokenstream::TokenStream;
    /// 
    /// let mut token_stream = TokenStream::new(&[0; 12]);
    /// token_stream.advance(12);
    /// 
    ///
    /// assert_eq!(token_stream.mark().position(), 12);
    /// ```
    pub fn position(&self) -> usize {
        self.position
    }
}
