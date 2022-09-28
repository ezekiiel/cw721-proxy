use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use cw_rate_limiter::RateLimiter;

pub const RATE_LIMIT: RateLimiter = RateLimiter::new("rate_limit", "last_updated", "this_block");
pub const ORIGIN: Item<Addr> = Item::new("origin");
