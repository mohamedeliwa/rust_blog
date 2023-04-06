/**
* We’ll show you how to rethink the state pattern to get a different set of trade-offs.
* Rather than encapsulating the states and transitions completely so outside code has no knowledge of them,
* we’ll encode the states into different types.
* Consequently, Rust’s type checking system will prevent attempts
* to use draft posts where only published posts are allowed by issuing a compiler error.
* https://rust-book.cs.brown.edu/ch17-03-oo-design-patterns.html
* **/
pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
