use std::collections::HashMap;

use crate::commodity::Commodity;

/**
 * pool.h
 * Types for managing commodity pools
 */

pub(crate) struct CommodityPool {
    pub commodities_map: HashMap<String, Commodity>,
    //annotated_commodities
    //commodity_price_history
}

impl CommodityPool {
    pub fn new() -> Self {
        Self { commodities_map: HashMap::new() }
    }

    pub fn find(&self, symbol: &str) -> Option<&Commodity> {
        self.commodities_map.get(symbol)
    }
}