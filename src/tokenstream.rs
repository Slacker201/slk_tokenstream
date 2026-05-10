use std::{collections::HashMap, sync::{Arc, atomic::AtomicU64}};

use crate::bookmark::{Inner, Mark};



/// A generic TokenStream struct that manages a stream of tokens with cursor and bookmark functionality.
#[derive(Debug)]
pub struct TokenStream<T> {
    data: Vec<T>,
    cursor: usize,
    bookmarks: HashMap<Inner, usize>,
    previous_bookmark: u64,
}


impl<T> TokenStream<T> {
    /// Creates a new TokenStream from a vector of tokens. Sets cursor to 0 and initializes an empty bookmark map.
    pub fn new(data: Vec<T>) -> Self {
        TokenStream {
            data,
            cursor: 0,
            bookmarks: HashMap::new(),
            previous_bookmark: 0,
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
    /// Moves the cursor back by one position, saturating at 0.
    pub fn rewind(&mut self) {
        self.rewind_offset(1);
    }
    /// Rewinds the cursor a specified amount of times, saturating at 0.
    pub fn rewind_offset(&mut self, offset: usize) {
        self.cursor = self.cursor.saturating_sub(offset);
    }
    /// Sets the cursor to a specific position, clamping it within the bounds of the data vector.
    pub fn set_cursor(&mut self, position: usize) {
        self.cursor = position.min(self.data.len());
    }
    /// Registers a bookmark at the current cursor position and returns a handle.
    pub fn mark(&mut self) -> Mark {
        let mark = Mark::new(self.previous_bookmark);
        self.bookmarks.insert(mark.inner.clone(), self.cursor);
        self.previous_bookmark += 1;
        mark
    }
    /// Moves the cursor to the position of a previously registered bookmark by handle. Returns the previous position if the bookmark is found.
    pub fn reset(&mut self, bookmark: &Mark) -> Option<usize> {
        if let Some(&position) = self.bookmarks.get(&bookmark.inner) {
            let prev_pos = self.cursor;
            self.cursor = position;
            Some(prev_pos)
        } else {
            None
        }
    }
    /// Removes the specified bookmark and returns whether it was found
    pub fn remove_mark(&mut self, bookmark: Mark) -> bool {
        self.bookmarks.remove(&bookmark.inner).is_some()
    }
    /// Removes unused bookmarks
    pub fn clean_bookmarks(&mut self) {
        let mut items_to_remove = Vec::with_capacity(self.bookmarks.len());
        for item in &self.bookmarks {
            if item.0.tracker_val() == 0 {
                items_to_remove.push(item.0.id());
            }
        }
        for id in items_to_remove {
            self.bookmarks.remove(&id.into());
        }
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