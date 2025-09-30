use std::collections::HashMap;



/// A generic TokenStream struct that manages a stream of tokens with cursor and bookmark functionality.
#[derive(Debug, Clone)]
pub struct TokenStream<T> {
    data: Vec<T>,
    cursor: usize,
    bookmarks: HashMap<String, usize>,
}


impl<T> TokenStream<T> {
    /// Creates a new TokenStream from a vector of tokens. Sets cursor to 0 and initializes an empty bookmark map.
    pub fn new(data: Vec<T>) -> Self {
        TokenStream {
            data,
            cursor: 0,
            bookmarks: HashMap::new(),
        }
    }
    /// Advances the cursor and returns the next token if available, otherwise returns None.
    pub fn consume(&mut self) -> Option<&T> {
        let val = self.data.get(self.cursor);
        if val.is_some() {
            self.cursor += 1;
        }
        return val;
    }

    /// Peeks at the token at the current cursor position plus an optional offset without advancing the cursor.
    pub fn peek(&self) -> Option<&T> {
        self.peek_offset(0)
    }
    pub fn peek_offset(&self, offset: usize) -> Option<&T> {
        self.data.get(self.cursor + offset)
    }
    /// Moves the cursor back by one position, ensuring it doesn't go below zero.
    pub fn rewind(&mut self) {
        self.rewind_offset(1);
    }
    /// Rewinds the cursor a specified amount of times, ensuring it doesn't go below zero.
    pub fn rewind_offset(&mut self, offset: usize) {
        self.cursor = self.cursor.saturating_sub(offset);
    }
    /// Sets the cursor to a specific position, clamping it within the bounds of the data vector.
    pub fn set_cursor(&mut self, position: usize) {
        self.cursor = position.min(self.data.len());
    }
    /// Registers a bookmark at the current cursor position with the given name.
    pub fn register_bookmark(&mut self, name: String) {
        self.bookmarks.insert(name, self.cursor);
    }
    /// Moves the cursor to the position of a previously registered bookmark by name. Returns true if successful, false if the bookmark doesn't exist.
    pub fn goto_bookmark(&mut self, name: &str) -> Option<usize> {
        if let Some(&position) = self.bookmarks.get(name) {
            let prev_pos = self.cursor;
            self.cursor = position;
            Some(prev_pos)
        } else {
            None
        }
    }
    /// Removes the specified bookmark and returns whether it was found
    pub fn remove_bookmark(&mut self, bookmark: &str) -> bool {
        self.bookmarks.remove(bookmark).is_some()
    }
    /// Returns the current position of the cursor.
    pub fn cursor(&self) -> usize {
        self.cursor
    }
}

impl<T> Iterator for TokenStream<T> where T: Clone {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume().cloned()
    }
}