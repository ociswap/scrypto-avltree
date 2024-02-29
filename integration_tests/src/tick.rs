use scrypto::prelude::*;

#[derive(ScryptoSbor, Clone, Debug)]
pub struct Tick {
    pub delta_liquidity: PreciseDecimal,
    pub total_liquidity: PreciseDecimal,
    pub price_sqrt: PreciseDecimal,
    pub x_fee_outside: PreciseDecimal,
    pub y_fee_outside: PreciseDecimal,
}
