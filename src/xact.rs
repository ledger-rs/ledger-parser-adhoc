use chrono::NaiveDate;

use crate::post::Post;

/**
 * xact.cc
 */

#[derive(Debug)]
pub struct Xact {
    // Item
    date: Option<NaiveDate>,
    // date_aux

    // Xact base
    // journal
    pub posts: Vec<Post>,

    // code: Option<String>,
    payee: String,

    // state
    // date: Option<Date>
    // date_aux:
    note: Option<String>, // pos: Option<Position>
                          // metadata: Option<string_map>
}

impl Xact {
    pub fn new(date: Option<NaiveDate>, payee: String, note: Option<String>) -> Self {
        // code: Option<String>

        Self {
            payee,
            note,
            posts: vec![],
            date,
        }
    }

    pub fn add_post(&mut self, post: Post) {
        self.posts.push(post);
    }

    pub fn finalize(&self) {
        // TODO: calculate balance

        // TODO: calculate conversion ratio

        // Add a pointer to each posting to their related accounts

        todo!("complete")
    }

    pub fn remove_post(&self) {
        todo!("complete")
    }

    pub fn lookup(&self) {
        todo!("complete")
    }
}
