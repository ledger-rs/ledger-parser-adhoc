use crate::{xact::Xact, account::Account, amount::Amount};

/**
 * posts.h + posts.cc
 */

#[derive(Debug, PartialEq)]
pub struct Post {
    // xact: &Xact,
    // account: &Account,
    // temporary solution, use String.
    pub account: String,

    pub amount: Amount,
    // amount_expr: Option<Expr>
    // cost: Option<Amount>
    // given_cost: Option<Amount>
    // assigned_amount: Option<Amount>

    // checkin
    // checkout

    // payee: Option<String>
}

impl Post {
    pub fn new() -> Self {
        Self { amount: Amount::new(), account: "".to_string() }
    }
}