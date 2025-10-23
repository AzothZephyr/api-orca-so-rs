use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Protocol information including TVL, volume, fees, and revenue
#[derive(Debug, Deserialize)]
pub struct ProtocolInfo {
    #[serde(rename = "fees24hUsdc")]
    pub fees_24h_usdc: String,
    #[serde(rename = "revenue24hUsdc")]
    pub revenue_24h_usdc: String,
    pub tvl: String,
    #[serde(rename = "volume24hUsdc")]
    pub volume_24h_usdc: String,
}

/// Statistics for a token.
#[derive(Debug, Deserialize)]
pub struct TokenStats {
    #[serde(rename = "24h")]
    pub h24: TokenVolume,
}

/// The volume of a token.
#[derive(Debug, Deserialize)]
pub struct TokenVolume {
    pub volume: String,
}

/// Detailed information about the Orca token.
#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    #[serde(rename = "circulatingSupply")]
    pub circulating_supply: String,
    pub description: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub name: String,
    pub price: String,
    pub stats: TokenStats,
    pub symbol: String,
    #[serde(rename = "totalSupply")]
    pub total_supply: String,
}

/// The circulating supply of the Orca token.
#[derive(Debug, Deserialize)]
pub struct CirculatingSupplyResponse {
    pub circulating_supply: String,
}

/// The total supply of the Orca token.
#[derive(Debug, Deserialize)]
pub struct TotalSupplyResponse {
    pub total_supply: String,
}

/// A paginated response from the API.
#[derive(Debug, Deserialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub meta: Meta,
}

/// Metadata for a paginated response.
#[derive(Debug, Deserialize)]
pub struct Meta {
    pub next: Option<String>,
    pub previous: Option<String>,
}

/// Information about a token.
#[derive(Debug, Deserialize)]
pub struct Token {
    pub address: String,
    pub decimals: u8,
    pub extensions: String, // todo: parse this string as json
    #[serde(rename = "freezeAuthority")]
    pub freeze_authority: Option<String>,
    #[serde(rename = "isInitialized")]
    pub is_initialized: bool,
    pub metadata: String, // todo: parse this string as json
    #[serde(rename = "mintAuthority")]
    pub mint_authority: Option<String>,
    #[serde(rename = "priceUsdc")]
    pub price_usdc: String,
    pub stats: String, // todo: parse this string as json
    pub supply: String,
    pub tags: String, // todo: parse this string as json
    #[serde(rename = "tokenProgram")]
    pub token_program: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "updatedEpoch")]
    pub updated_epoch: u64,
}

/// Information about locked liquidity.
#[derive(Debug, Deserialize)]
pub struct LockInfo {
    #[serde(rename = "lockedPercentage")]
    pub locked_percentage: String,
    pub name: String,
}

/// A time period for statistics.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum TimePeriod {
    #[serde(rename = "5m")]
    M5,
    #[serde(rename = "15m")]
    M15,
    #[serde(rename = "30m")]
    M30,
    #[serde(rename = "1h")]
    H1,
    #[serde(rename = "2h")]
    H2,
    #[serde(rename = "4h")]
    H4,
    #[serde(rename = "8h")]
    H8,
    #[serde(rename = "12h")]
    H12,
    #[serde(rename = "24h")]
    H24,
}

/// Information about a whirlpool.
#[derive(Debug, Deserialize)]
pub struct Whirlpool {
    pub address: String,
    #[serde(rename = "feeGrowthGlobalA")]
    pub fee_growth_global_a: String,
    #[serde(rename = "feeGrowthGlobalB")]
    pub fee_growth_global_b: String,
    #[serde(rename = "feeRate")]
    pub fee_rate: u32,
    pub liquidity: String,
    #[serde(rename = "protocolFeeOwedA")]
    pub protocol_fee_owed_a: String,
    #[serde(rename = "protocolFeeOwedB")]
    pub protocol_fee_owed_b: String,
    #[serde(rename = "protocolFeeRate")]
    pub protocol_fee_rate: u32,
    #[serde(rename = "rewardLastUpdatedTimestamp")]
    pub reward_last_updated_timestamp: String,
    #[serde(rename = "sqrtPrice")]
    pub sqrt_price: String,
    #[serde(rename = "tickCurrentIndex")]
    pub tick_current_index: i32,
    #[serde(rename = "tickSpacing")]
    pub tick_spacing: u16,
    #[serde(rename = "tickSpacingSeed")]
    pub tick_spacing_seed: String,
    #[serde(rename = "tokenMintA")]
    pub token_mint_a: String,
    #[serde(rename = "tokenMintB")]
    pub token_mint_b: String,
    #[serde(rename = "tokenVaultA")]
    pub token_vault_a: Vec<u64>,
    #[serde(rename = "tokenVaultB")]
    pub token_vault_b: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "updatedSlot")]
    pub updated_slot: u64,
    #[serde(rename = "whirlpoolBump")]
    pub whirlpool_bump: String,
    #[serde(rename = "whirlpoolsConfig")]
    pub whirlpools_config: String,
    #[serde(rename = "writeVersion")]
    pub write_version: String,
    #[serde(rename = "adaptiveFee")]
    pub adaptive_fee: Option<AdaptiveFee>,
    #[serde(rename = "adaptiveFeeEnabled")]
    pub adaptive_fee_enabled: bool,
    #[serde(rename = "addressLookupTable")]
    pub address_lookup_table: Vec<u64>,
    #[serde(rename = "feeTierIndex")]
    pub fee_tier_index: u32,
    #[serde(rename = "hasWarning")]
    pub has_warning: bool,
    #[serde(rename = "lockedLiquidityPercent")]
    pub locked_liquidity_percent: Option<Vec<LockInfo>>,
    #[serde(rename = "poolType")]
    pub pool_type: String,
    pub price: String,
    pub rewards: Vec<Reward>,
    pub stats: HashMap<TimePeriod, PoolStats>,
    #[serde(rename = "tokenA")]
    pub token_a: SimpleTokenInfo,
    #[serde(rename = "tokenB")]
    pub token_b: SimpleTokenInfo,
    #[serde(rename = "tokenBalanceA")]
    pub token_balance_a: String,
    #[serde(rename = "tokenBalanceB")]
    pub token_balance_b: String,
    #[serde(rename = "tradeEnableTimestamp")]
    pub trade_enable_timestamp: String,
    #[serde(rename = "tvlUsdc")]
    pub tvl_usdc: String,
    #[serde(rename = "yieldOverTvl")]
    pub yield_over_tvl: String,
}

/// Information about adaptive fees.
#[derive(Debug, Deserialize)]
pub struct AdaptiveFee {
    pub constants: AdaptiveFeeConstants,
    #[serde(rename = "currentRate")]
    pub current_rate: u32,
    #[serde(rename = "maxRate")]
    pub max_rate: u32,
    pub variables: AdaptiveFeeVariables,
}

/// Constants for adaptive fees.
#[derive(Debug, Deserialize)]
pub struct AdaptiveFeeConstants {
    #[serde(rename = "adaptiveFeeControlFactor")]
    pub adaptive_fee_control_factor: u32,
    #[serde(rename = "decayPeriod")]
    pub decay_period: u32,
    #[serde(rename = "filterPeriod")]
    pub filter_period: u32,
    #[serde(rename = "majorSwapThresholdTicks")]
    pub major_swap_threshold_ticks: u32,
    #[serde(rename = "maxVolatilityAccumulator")]
    pub max_volatility_accumulator: u32,
    #[serde(rename = "reductionFactor")]
    pub reduction_factor: u32,
    #[serde(rename = "tickGroupSize")]
    pub tick_group_size: u32,
}

/// Variables for adaptive fees.
#[derive(Debug, Deserialize)]
pub struct AdaptiveFeeVariables {
    #[serde(rename = "lastMajorSwapTimestamp")]
    pub last_major_swap_timestamp: String,
    #[serde(rename = "lastReferenceUpdateTimestamp")]
    pub last_reference_update_timestamp: String,
    #[serde(rename = "tickGroupIndexReference")]
    pub tick_group_index_reference: i32,
    #[serde(rename = "volatilityAccumulator")]
    pub volatility_accumulator: u32,
    #[serde(rename = "volatilityReference")]
    pub volatility_reference: u32,
}

/// Information about a reward.
#[derive(Debug, Deserialize)]
pub struct Reward {
    pub authority: String,
    pub emissions_per_second_x64: String,
    pub growth_global_x64: String,
    pub mint: String,
    pub vault: String,
    pub active: bool,
    #[serde(rename = "emissionsPerSecond")]
    pub emissions_per_second: String,
}

/// Statistics for a pool.
#[derive(Debug, Deserialize)]
pub struct PoolStats {
    pub fees: String,
    pub rewards: String,
    pub volume: String,
    #[serde(rename = "yieldOverTvl")]
    pub yield_over_tvl: String,
}

/// Basic information about a token.
#[derive(Debug, Deserialize)]
pub struct SimpleTokenInfo {
    pub address: String,
    pub decimals: u8,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub name: String,
    #[serde(rename = "programId")]
    pub program_id: String,
    pub symbol: String,
    pub tags: String, // todo: parse as json
}