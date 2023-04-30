use crate::{xact::Xact, account::Account, amount::Amount};

/**
 * posts.h + posts.cc
 */

pub struct Post<'a> {
    xact: &'a Xact<'a>,
    account: &'a Account,

    amount: Amount,
    // amount_expr
    // cost
    // given_cost
    // assigned_amount

    // timestamps:
    // checkin
    // checkout

    // payee: Option<String>
}