use chrono::NaiveDate;

use crate::post::Post;

/**
 * xact.cc
 */

pub(crate) struct Xact {
    // Item
    date: Option<NaiveDate>,
    // date_aux

    // Xact base
    // journal
    posts: Vec<Post>,

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

    pub fn add_post() {
        todo!("implement")
    }

    pub fn remove_post() {
        todo!("complete")
    }

    pub fn lookup() {
        todo!("complete")
    }
}
