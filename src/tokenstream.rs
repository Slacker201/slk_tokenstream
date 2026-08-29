mod consume_methods;
mod cursor_methods;
mod mark_methods;
mod peek_methods;
mod skip_methods;

/// A generic TokenStream struct that manages a stream of tokens with cursor and bookmark functionality.
///
/// # Examples
///
/// ``` rust
/// use slk_tokenstream::TokenStream;
/// use slk_tokenstream::Mark;
///
/// let tokens = &[1, 2, 3];
/// let mut token_stream = TokenStream::new(tokens);
///
/// assert_eq!(token_stream.consume(), Some(&1));
/// assert_eq!(token_stream.peek(), Some(&2));
/// assert_eq!(token_stream.tokens_remaining(), 2);
/// ```
#[derive(Debug)]
pub struct TokenStream<'a, T> {
    data: &'a [T],
    cursor: usize,
}

impl<'a, T> TokenStream<'a, T> {
    /// Creates a new TokenStream from a vector of tokens. Sets cursor to 0.
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
    pub fn new(data: &'a [T]) -> Self {
        TokenStream { data, cursor: 0 }
    }
}
