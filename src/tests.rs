#[test]
fn comsume_returns_items_and_advances_cursor() {
    let mut ts = crate::TokenStream::new(&[1, 2, 3]);
    assert_eq!(ts.consume(), Some(&1));
    assert_eq!(ts.position(), 1);
    assert_eq!(ts.consume(), Some(&2));
    assert_eq!(ts.position(), 2);
    assert_eq!(ts.consume(), Some(&3));
    assert_eq!(ts.position(), 3);
    assert_eq!(ts.consume(), None);
    assert_eq!(ts.position(), 3);
}
#[test]
fn peek_offset_reads_the_correct_index() {
    let ts = crate::TokenStream::new(&[1, 2, 3]);
    assert_eq!(ts.peek_offset(0), Some(&1));
    assert_eq!(ts.peek_offset(1), Some(&2));
    assert_eq!(ts.peek_offset(2), Some(&3));
    assert_eq!(ts.peek_offset(3), None);
}
#[test]
fn peek_reads_the_correct_index() {
    let mut ts = crate::TokenStream::new(&[1, 2, 3]);
    assert_eq!(ts.peek(), Some(&1));
    ts.advance(1);
    assert_eq!(ts.peek(), Some(&2));
}
#[test]
fn rewind_moves_index_back_by_one() {
    let mut ts = crate::TokenStream::new(&[1, 2, 3]);
    ts.advance(2);
    assert_eq!(ts.position(), 2);
    ts.rewind();
    assert_eq!(ts.position(), 1);
}
#[test]
fn bookmarks_correctly_return_cursor_to_original_position() {
    let mut ts = crate::TokenStream::new(&[1, 2, 3]);
    ts.consume();
    let mark = ts.mark();
    ts.consume();
    let mark2 = ts.mark();
    ts.consume();
    assert_eq!(ts.reset(&mark), 3);
    assert_eq!(ts.position(), 1);
    ts.reset(&mark2);
    assert_eq!(ts.position(), 2);
    assert_eq!(ts.consume(), Some(&3));
}

#[test]
fn rewind_does_not_underflow() {
    let mut ts = crate::TokenStream::new(&[1, 2, 3]);
    assert_eq!(ts.position(), 0);
    ts.rewind();
    assert_eq!(ts.position(), 0);
}

#[test]
fn rewind_offset_does_not_underflow() {
    let mut ts = crate::TokenStream::new(&[1, 2, 3]);
    assert_eq!(ts.position(), 0);
    ts.rewind_offset(123);
    assert_eq!(ts.position(), 0);
}

#[test]
fn advance_does_not_move_cursor_past_eof() {
    let mut ts = crate::TokenStream::new(&[1, 2, 3]);
    assert_eq!(ts.position(), 0);
    ts.advance(123);
    assert_eq!(ts.position(), 3);
}