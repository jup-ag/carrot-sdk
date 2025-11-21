// calculate the shares earned from depositing the usd
pub fn shares_earned(
    usd_value: u128,
    shares_supply: u64,
    shares_decimals: u8,
    vault_tvl: u128,
    round_up: bool,
) -> Option<u64> {
    if vault_tvl.le(&0) || shares_supply.le(&0) {
        // if vault_tvl or shares_supply is 0 or less, just mint 100 shares
        ui_to_amount(100.0, shares_decimals)
    } else {
        // Calculate shares
        let shares = if round_up {
            let prod = usd_value.checked_mul(shares_supply as u128)?;
            let sum = prod.checked_add(vault_tvl - 1)?;
            sum.checked_div(vault_tvl)?
        } else {
            let prod = usd_value.checked_mul(shares_supply as u128)?;
            prod.checked_div(vault_tvl)?
        };

        // Check if the result fits within u64
        if shares > u64::MAX as u128 {
            Some(u64::MAX) // Handle overflow case, e.g., by returning the maximum u64 value
        } else {
            Some(shares as u64)
        }
    }
}

pub fn usd_earned(shares_to_redeem: u64, shares_supply: u64, vault_tvl: u128) -> Option<u128> {
    if vault_tvl.le(&0) || shares_supply.le(&0) {
        // if vault_tvl or shares_supply is 0 or less, return 0 USD
        Some(0)
    } else {
        // rounds down
        let usd_earned: u128 = (shares_to_redeem as u128)
            .checked_mul(vault_tvl)?
            .checked_div(shares_supply as u128)?;

        Some(usd_earned)
    }
}

pub fn calc_usd_amount(
    token_amount: u64,
    token_decimal: u8,
    price_feed_price: i64,
    price_feed_expo: i32,
    ceiling: bool,
) -> Option<u128> {
    if price_feed_expo >= 0 {
        return None;
    }

    let token_amount = token_amount as u128;
    let price_feed_price = price_feed_price.unsigned_abs() as u128;

    // Scale the token amount to the base unit (USD cents)
    let decimal_adjustment = PRECISION.checked_sub(token_decimal)? as u32;
    let scale_multiplier = 10_u128.checked_pow(decimal_adjustment)?;
    let scaled_token_amount = token_amount.checked_mul(scale_multiplier)?;

    // Perform safe multiplication to get numerator
    let numerator = scaled_token_amount.checked_mul(price_feed_price)?;

    {
        let expo = price_feed_expo.checked_neg()? as u32;
        let divisor = 10_u128.checked_pow(expo)?;

        if ceiling {
            // Adjust for ceiling by adding divisor - 1 before division
            let adjusted_result = numerator
                .checked_add(divisor.checked_sub(1)?)?
                .checked_div(divisor)?;
            Some(adjusted_result)
        } else {
            // Direct division for floor rounding
            let adjusted_result = numerator.checked_div(divisor)?;
            Some(adjusted_result)
        }
    }
}

pub fn calc_token_amount(
    scaled_usd_amount: u128,
    token_decimal: u8,
    price_feed_price: i64,
    price_feed_expo: i32,
    ceiling: bool,
) -> Option<u64> {
    if price_feed_expo >= 0 {
        return None;
    }

    let price_exponent = price_feed_expo.checked_neg()? as u32;
    let price_feed_price = price_feed_price.unsigned_abs() as u128;
    if price_feed_price == 0 {
        return None;
    }

    // Handle exponent adjustment for result based on the expo sign
    let result = {
        let multiplier = 10_u128.checked_pow(price_exponent)?;
        let temp_result = scaled_usd_amount.checked_mul(multiplier)?;
        let adjusted_result = if ceiling {
            let increment = price_feed_price.checked_sub(1)?;
            temp_result.checked_add(increment)?
        } else {
            temp_result
        };
        adjusted_result.checked_div(price_feed_price)
    }?;

    // Adjust for token decimals
    let decimal_adjustment = PRECISION.checked_sub(token_decimal)? as u32;
    let divisor = 10_u128.checked_pow(decimal_adjustment)?;
    let token_amount = if ceiling {
        result
            .checked_add(divisor.checked_sub(1)?)?
            .checked_div(divisor)
    } else {
        result.checked_div(divisor)
    }?;

    // Ensure the result fits within u64
    if token_amount > u64::MAX as u128 {
        None
    } else {
        Some(token_amount as u64)
    }
}

const PRECISION: u8 = 9;

fn ui_to_amount(ui: f64, decimal: u8) -> Option<u64> {
    // Convert the floating-point to an integer without losing precision
    let multiplier = 10u64.checked_pow(decimal as u32)?;
    let ui_int = (ui * multiplier as f64).round() as u64;

    // Check for overflow during conversion
    ui_int.checked_mul(multiplier)
}
