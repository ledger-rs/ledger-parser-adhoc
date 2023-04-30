/**
 * xact.cc
 */

pub(crate) struct Xact {
    code: Option<String>,
    payee: String,

    // state
    // date: Option<Date>
    // date_aux: 
    note: Option<String>
    // pos: Option<Position>
    // metadata: Option<string_map>
}

impl Xact {
    pub fn new(code: Option<String>, payee: String, note: Option<String>) -> Self {
        Self { code, payee, note }
    }

    pub fn add_post() {
        todo!("implement")
    }

    pub fn lookup() {
        todo!("complete")
    }
}