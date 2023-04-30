use crate::commodity::Commodity;

/**
 * amount.h + amount.cc
 */

pub struct Amount {
    precision: u16,

    quantity: i64,
    commodity: Commodity
}