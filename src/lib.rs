
pub mod tokenstream;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_works() {
        let mut ts = tokenstream::TokenStream::new(vec![1, 2, 3]);
        assert_eq!(ts.consume(), Some(&1));
        assert_eq!(ts.consume(), Some(&2));
        assert_eq!(ts.consume(), Some(&3));
        assert_eq!(ts.consume(), None);
    }
    #[test]
    fn peek_works() {
        let ts = tokenstream::TokenStream::new(vec![1, 2, 3]);
        assert_eq!(ts.peek_offset(0), Some(&1));
        assert_eq!(ts.peek_offset(1), Some(&2));
        assert_eq!(ts.peek_offset(2), Some(&3));
        assert_eq!(ts.peek_offset(3), None);
    }
    #[test]
    fn rewind_works() {
        let mut ts = tokenstream::TokenStream::new(vec![1, 2, 3]);
        ts.consume();
        ts.consume();
        ts.rewind();
        assert_eq!(ts.consume(), Some(&2));
    }
    #[test]
    fn set_cursor_works() {
        let mut ts = tokenstream::TokenStream::new(vec![1, 2, 3]);
        ts.set_cursor(2);
        assert_eq!(ts.consume(), Some(&3));
        ts.set_cursor(10);
        assert_eq!(ts.consume(), None);
    }
    #[test]
    fn bookmarks_work() {
        let mut ts = tokenstream::TokenStream::new(vec![1, 2, 3]);
        ts.consume();
        ts.register_bookmark("first".to_string());
        ts.consume();
        ts.register_bookmark("second".to_string());
        ts.consume();
        assert_eq!(ts.goto_bookmark("first"), Some(3));
        assert_eq!(ts.consume(), Some(&2));
        assert_eq!(ts.goto_bookmark("second"), Some(2));
        assert_eq!(ts.consume(), Some(&3));
        assert_eq!(ts.goto_bookmark("nonexistent"), None);
    }

}
