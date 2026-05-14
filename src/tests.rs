#[test]
fn next_works() {
    let mut ts = crate::TokenStream::new(vec![1, 2, 3]);
    assert_eq!(ts.consume(), Some(&1));
    assert_eq!(ts.consume(), Some(&2));
    assert_eq!(ts.consume(), Some(&3));
    assert_eq!(ts.consume(), None);
}
#[test]
fn peek_works() {
    let ts = crate::TokenStream::new(vec![1, 2, 3]);
    assert_eq!(ts.peek_offset(0), Some(&1));
    assert_eq!(ts.peek_offset(1), Some(&2));
    assert_eq!(ts.peek_offset(2), Some(&3));
    assert_eq!(ts.peek_offset(3), None);
}
#[test]
fn rewind_works() {
    let mut ts = crate::TokenStream::new(vec![1, 2, 3]);
    ts.consume();
    ts.consume();
    ts.rewind();
    assert_eq!(ts.consume(), Some(&2));
}
#[test]
fn bookmarks_work() {
    let mut ts = crate::TokenStream::new(vec![1, 2, 3]);
    ts.consume();
    let mark = ts.mark();
    ts.consume();
    let mark2 = ts.mark();
    ts.consume();
    assert_eq!(ts.reset(&mark), 3);
    assert_eq!(ts.consume(), Some(&2));
    assert_eq!(ts.reset(&mark2), 2);
    assert_eq!(ts.consume(), Some(&3));
}