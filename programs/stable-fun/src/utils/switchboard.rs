use anchor_lang::prelude::*;
use switchboard_v2::AggregatorAccountData;
use crate::error::StableFunError;

#[inline(always)]
pub fn load_switchboard_feed<'a>(
    feed: &'a AccountLoader<AggregatorAccountData>
) -> Result<std::cell::Ref<'a, AggregatorAccountData>> {
    feed.load()
}

#[inline(always)]
pub fn get_feed_result(
    feed: &AggregatorAccountData
) -> Result<u64> {
    let price = feed.latest_confirmed_round.result;
    require!(price.mantissa > 0, StableFunError::InvalidOraclePrice);
    
    // Convert to u64 with proper scaling
    let price_u64 = price.mantissa as u64;
    Ok(price_u64)
}

#[inline(always)]
pub fn validate_feed_data(
    feed: &AggregatorAccountData,
    max_staleness: i64
) -> Result<()> {
    let current_timestamp = Clock::get()?.unix_timestamp;
    let last_update = feed.latest_confirmed_round.round_open_timestamp;
    
    require!(
        current_timestamp - last_update <= max_staleness,
        StableFunError::StaleOraclePrice
    );
    
    Ok(())
}

#[inline(always)]
pub fn get_validated_price(
    feed: &AccountLoader<AggregatorAccountData>,
    max_staleness: i64
) -> Result<u64> {
    let feed_data = load_switchboard_feed(feed)?;
    validate_feed_data(&feed_data, max_staleness)?;
    get_feed_result(&feed_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_price_validation() {
        // Test implementations will go here
    }
}