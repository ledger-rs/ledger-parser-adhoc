use chrono::NaiveDate;

use crate::post::Post;

/**
 * xact.cc
 */

pub(crate) struct Xact<'a> {
    // Item
    date: Option<NaiveDate>,
    // date_aux

    // Xact base
    // journal
    posts: Vec<Post<'a>>,

    // code: Option<String>,
    payee: &'a str,

    // state
    // date: Option<Date>
    // date_aux:
    note: Option<&'a str>, // pos: Option<Position>
                           // metadata: Option<string_map>
}

impl<'a> Xact<'a> {
    pub fn new(date: Option<NaiveDate>, payee: &'a str, note: Option<&'a str>) -> Self {
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
