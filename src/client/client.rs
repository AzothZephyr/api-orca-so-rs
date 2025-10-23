use crate::models::models::{
    CirculatingSupplyResponse, LockInfo, Paginated, ProtocolInfo, TimePeriod, Token, TokenInfo,
    TotalSupplyResponse, Whirlpool,
};
use reqwest::{Client, Url};
use std::error::Error;

const BASE_URL: &str = "https://api.orca.so/v2";

/// The main client for interacting with the Orca Public API.
pub struct OrcaClient {
    client: Client,
    base_url: String,
}

/// Parameters for the `get_pools` endpoint.
#[derive(Default)]
pub struct GetPoolsParams<'a> {
    pub sort_by: Option<&'a str>,
    pub sort_direction: Option<&'a str>,
    pub next: Option<&'a str>,
    pub previous: Option<&'a str>,
    pub has_rewards: Option<bool>,
    pub has_warning: Option<bool>,
    pub has_adaptive_fee: Option<bool>,
    pub is_wavebreak: Option<bool>,
    pub min_tvl: Option<f64>,
    pub min_volume: Option<f64>,
    pub min_locked_liquidity_percent: Option<f64>,
    pub size: Option<u32>,
    pub token: Option<&'a [u64]>,
    pub tokens_both_of: Option<&'a [&'a str]>,
    pub addresses: Option<&'a [&'a str]>,
    pub stats: Option<&'a [TimePeriod]>,
    pub include_blocked: Option<bool>,
}

#[derive(Default)]
/// Parameters for the `search_pools` endpoint.
pub struct SearchPoolsParams<'a> {
    pub q: &'a str,
    pub next: Option<&'a str>,
    pub size: Option<u32>,
    pub sort_by: Option<&'a str>,
    pub sort_direction: Option<&'a str>,
    pub min_tvl: Option<f64>,
    pub min_volume: Option<f64>,
    pub stats: Option<&'a [TimePeriod]>,
    pub user_tokens: Option<&'a [&'a str]>,
    pub has_rewards: Option<bool>,
    pub verified_only: Option<bool>,
    pub has_locked_liquidity: Option<bool>,
}

impl OrcaClient {
    /// Creates a new `OrcaClient` with the default base URL.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    /// Creates a new `OrcaClient` with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// Returns general information about the Orca protocol.
    pub async fn get_protocol_info(&self, chain: &str) -> Result<ProtocolInfo, Box<dyn Error>> {
        let url = format!("{}/{}/protocol", self.base_url, chain);
        let response = self.client.get(&url).send().await?;
        let protocol_info = response.json::<ProtocolInfo>().await?;
        Ok(protocol_info)
    }

    /// Returns detailed information about the Orca token.
    pub async fn get_token_info(&self, chain: &str) -> Result<TokenInfo, Box<dyn Error>> {
        let url = format!("{}/{}/protocol/token", self.base_url, chain);
        let response = self.client.get(&url).send().await?;
        let token_info = response.json::<TokenInfo>().await?;
        Ok(token_info)
    }

    /// Returns the circulating supply of the protocol's token.
    pub async fn get_circulating_supply(
        &self,
        chain: &str,
    ) -> Result<CirculatingSupplyResponse, Box<dyn Error>> {
        let url = format!("{}/{}/protocol/token/circulating_supply", self.base_url, chain);
        let response = self.client.get(&url).send().await?;
        let circulating_supply = response.json::<CirculatingSupplyResponse>().await?;
        Ok(circulating_supply)
    }

    /// Returns the total supply of the protocol's token.
    pub async fn get_total_supply(
        &self,
        chain: &str,
    ) -> Result<TotalSupplyResponse, Box<dyn Error>> {
        let url = format!("{}/{}/protocol/token/total_supply", self.base_url, chain);
        let response = self.client.get(&url).send().await?;
        let total_supply = response.json::<TotalSupplyResponse>().await?;
        Ok(total_supply)
    }

    /// Returns a paginated list of tokens with optional filtering and sorting.
    pub async fn get_tokens<'a>(
        &self,
        chain: &str,
        next: Option<&'a str>,
        previous: Option<&'a str>,
        size: Option<u32>,
        sort_by: Option<&'a str>,
        sort_direction: Option<&'a str>,
        tokens: Option<&'a str>,
    ) -> Result<Paginated<Token>, Box<dyn Error>> {
        let mut url = Url::parse(&format!("{}/{}/tokens", self.base_url, chain))?;

        if let Some(next) = next {
            url.query_pairs_mut().append_pair("next", next);
        }
        if let Some(previous) = previous {
            url.query_pairs_mut().append_pair("previous", previous);
        }
        if let Some(size) = size {
            url.query_pairs_mut()
                .append_pair("size", &size.to_string());
        }
        if let Some(sort_by) = sort_by {
            url.query_pairs_mut().append_pair("sort_by", sort_by);
        }
        if let Some(sort_direction) = sort_direction {
            url.query_pairs_mut()
                .append_pair("sort_direction", sort_direction);
        }
        if let Some(tokens) = tokens {
            url.query_pairs_mut().append_pair("tokens", tokens);
        }

        let response = self.client.get(url).send().await?;
        let tokens = response.json::<Paginated<Token>>().await?;
        Ok(tokens)
    }

    /// Returns a list of tokens that match the query string.
    pub async fn search_tokens(
        &self,
        chain: &str,
        query: &str,
    ) -> Result<Paginated<Token>, Box<dyn Error>> {
        let mut url = Url::parse(&format!("{}/{}/tokens/search", self.base_url, chain))?;
        url.query_pairs_mut().append_pair("q", query);

        let response = self.client.get(url).send().await?;
        let tokens = response.json::<Paginated<Token>>().await?;
        Ok(tokens)
    }

    /// Returns detailed information for a specific token identified by its mint address.
    pub async fn get_token(
        &self,
        chain: &str,
        mint_address: &str,
    ) -> Result<Paginated<Token>, Box<dyn Error>> {
        let url = format!("{}/{}/tokens/{}", self.base_url, chain, mint_address);
        let response = self.client.get(&url).send().await?;
        let token = response.json::<Paginated<Token>>().await?;
        Ok(token)
    }

    /// This endpoint returns the locked liquidity for a given whirlpool.
    pub async fn get_lock_info(
        &self,
        chain: &str,
        address: &str,
    ) -> Result<Vec<LockInfo>, Box<dyn Error>> {
        let url = format!("{}/{}/lock/{}", self.base_url, chain, address);
        let response = self.client.get(&url).send().await?;
        let lock_info = response.json::<Vec<LockInfo>>().await?;
        Ok(lock_info)
    }

    /// List whirlpools with optional filtering and pagination
    pub async fn get_pools<'a>(
        &self,
        chain: &str,
        params: GetPoolsParams<'a>,
    ) -> Result<Paginated<Whirlpool>, Box<dyn Error>> {
        let mut url = Url::parse(&format!("{}/{}/pools", self.base_url, chain))?;
        let mut query_pairs = url.query_pairs_mut();

        if let Some(sort_by) = params.sort_by {
            query_pairs.append_pair("sortBy", sort_by);
        }
        if let Some(sort_direction) = params.sort_direction {
            query_pairs.append_pair("sortDirection", sort_direction);
        }
        if let Some(next) = params.next {
            query_pairs.append_pair("next", next);
        }
        if let Some(previous) = params.previous {
            query_pairs.append_pair("previous", previous);
        }
        if let Some(has_rewards) = params.has_rewards {
            query_pairs.append_pair("hasRewards", &has_rewards.to_string());
        }
        if let Some(has_warning) = params.has_warning {
            query_pairs.append_pair("hasWarning", &has_warning.to_string());
        }
        if let Some(has_adaptive_fee) = params.has_adaptive_fee {
            query_pairs.append_pair("hasAdaptiveFee", &has_adaptive_fee.to_string());
        }
        if let Some(is_wavebreak) = params.is_wavebreak {
            query_pairs.append_pair("isWavebreak", &is_wavebreak.to_string());
        }
        if let Some(min_tvl) = params.min_tvl {
            query_pairs.append_pair("minTvl", &min_tvl.to_string());
        }
        if let Some(min_volume) = params.min_volume {
            query_pairs.append_pair("minVolume", &min_volume.to_string());
        }
        if let Some(min_locked_liquidity_percent) = params.min_locked_liquidity_percent {
            query_pairs.append_pair(
                "minLockedLiquidityPercent",
                &min_locked_liquidity_percent.to_string(),
            );
        }
        if let Some(size) = params.size {
            query_pairs.append_pair("size", &size.to_string());
        }
        if let Some(token) = params.token {
            for t in token {
                query_pairs.append_pair("token", &t.to_string());
            }
        }
        if let Some(tokens_both_of) = params.tokens_both_of {
            for t in tokens_both_of {
                query_pairs.append_pair("tokensBothOf", t);
            }
        }
        if let Some(addresses) = params.addresses {
            for a in addresses {
                query_pairs.append_pair("addresses", a);
            }
        }
        if let Some(stats) = params.stats {
            for s in stats {
                query_pairs.append_pair(
                    "stats",
                    &serde_json::to_string(s)
                        .unwrap_or_default()
                        .replace('"', ""),
                );
            }
        }
        if let Some(include_blocked) = params.include_blocked {
            query_pairs.append_pair("includeBlocked", &include_blocked.to_string());
        }

        drop(query_pairs);

        let response = self.client.get(url).send().await?;
        let pools = response.json::<Paginated<Whirlpool>>().await?;
        Ok(pools)
    }

    /// This endpoint allows searching for whirlpools
    pub async fn search_pools<'a>(
        &self,
        chain: &str,
        params: SearchPoolsParams<'a>,
    ) -> Result<Paginated<Whirlpool>, Box<dyn Error>> {
        let mut url = Url::parse(&format!("{}/{}/pools/search", self.base_url, chain))?;
        let mut query_pairs = url.query_pairs_mut();

        query_pairs.append_pair("q", params.q);

        if let Some(next) = params.next {
            query_pairs.append_pair("next", next);
        }
        if let Some(size) = params.size {
            query_pairs.append_pair("size", &size.to_string());
        }
        if let Some(sort_by) = params.sort_by {
            query_pairs.append_pair("sortBy", sort_by);
        }
        if let Some(sort_direction) = params.sort_direction {
            query_pairs.append_pair("sortDirection", sort_direction);
        }
        if let Some(min_tvl) = params.min_tvl {
            query_pairs.append_pair("minTvl", &min_tvl.to_string());
        }
        if let Some(min_volume) = params.min_volume {
            query_pairs.append_pair("minVolume", &min_volume.to_string());
        }
        if let Some(stats) = params.stats {
            for s in stats {
                query_pairs.append_pair(
                    "stats",
                    &serde_json::to_string(s)
                        .unwrap_or_default()
                        .replace('"', ""),
                );
            }
        }
        if let Some(user_tokens) = params.user_tokens {
            for t in user_tokens {
                query_pairs.append_pair("userTokens", t);
            }
        }
        if let Some(has_rewards) = params.has_rewards {
            query_pairs.append_pair("hasRewards", &has_rewards.to_string());
        }
        if let Some(verified_only) = params.verified_only {
            query_pairs.append_pair("verifiedOnly", &verified_only.to_string());
        }
        if let Some(has_locked_liquidity) = params.has_locked_liquidity {
            query_pairs.append_pair("hasLockedLiquidity", &has_locked_liquidity.to_string());
        }

        drop(query_pairs);
        let response = self.client.get(url).send().await?;
        let pools = response.json::<Paginated<Whirlpool>>().await?;
        Ok(pools)
    }

    /// Get whirlpool data by address
    pub async fn get_pool(
        &self,
        chain: &str,
        address: &str,
    ) -> Result<Paginated<Whirlpool>, Box<dyn Error>> {
        let url = format!("{}/{}/pools/{}", self.base_url, chain, address);
        let response = self.client.get(&url).send().await?;
        let pool = response.json::<Paginated<Whirlpool>>().await?;
        Ok(pool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_protocol_info() {
        let _m = mock("GET", "/solana/protocol")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "fees24hUsdc": "317428.0521046",
                    "revenue24hUsdc": "41265.646773",
                    "tvl": "230551269.0085",
                    "volume24hUsdc": "552567794.7830"
                }"#,
            )
            .create();

        let client = OrcaClient::with_base_url(&mockito::server_url());
        let result = client.get_protocol_info("solana").await;

        assert!(result.is_ok());
        let protocol_info = result.unwrap();
        assert_eq!(protocol_info.fees_24h_usdc, "317428.0521046");
    }

    #[tokio::test]
    async fn test_get_token_info() {
        let _m = mock("GET", "/solana/protocol/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "circulatingSupply": "53275182.419413",
                    "description": "Orca Token",
                    "imageUrl": "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE/logo.png",
                    "name": "Orca",
                    "price": "1.6767140",
                    "stats": {
                        "24h": {
                            "volume": "594947.6898176792"
                        }
                    },
                    "symbol": "ORCA",
                    "totalSupply": "99999712.243267"
                }"#,
            )
            .create();

        let client = OrcaClient::with_base_url(&mockito::server_url());
        let result = client.get_token_info("solana").await;

        assert!(result.is_ok());
        let token_info = result.unwrap();
        assert_eq!(token_info.name, "Orca");
    }

    #[tokio::test]
    async fn test_get_circulating_supply() {
        let _m = mock("GET", "/solana/protocol/token/circulating_supply")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"circulating_supply": "53275183"}"#)
            .create();

        let client = OrcaClient::with_base_url(&mockito::server_url());
        let result = client.get_circulating_supply("solana").await;

        assert!(result.is_ok());
        let circulating_supply = result.unwrap();
        assert_eq!(circulating_supply.circulating_supply, "53275183");
    }

    #[tokio::test]
    async fn test_get_total_supply() {
        let _m = mock("GET", "/solana/protocol/token/total_supply")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"total_supply": "99999713"}"#)
            .create();

        let client = OrcaClient::with_base_url(&mockito::server_url());
        let result = client.get_total_supply("solana").await;

        assert!(result.is_ok());
        let total_supply = result.unwrap();
        assert_eq!(total_supply.total_supply, "99999713");
    }

    #[tokio::test]
    async fn test_get_tokens() {
        let _m = mock("GET", "/solana/tokens?size=1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "data": [
                        {
                            "address": "So11111111111111111111111111111111111111112",
                            "decimals": 9,
                            "extensions": "{}",
                            "freezeAuthority": null,
                            "isInitialized": true,
                            "metadata": "{}",
                            "mintAuthority": null,
                            "priceUsdc": "130.0",
                            "stats": "{}",
                            "supply": "1000000000",
                            "tags": "[]",
                            "tokenProgram": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                            "updatedAt": "2025-05-09T00:04:50.745163Z",
                            "updatedEpoch": 784
                        }
                    ],
                    "meta": {
                        "next": "some-next-cursor",
                        "previous": null
                    }
                }"#,
            )
            .create();

        let client = OrcaClient::with_base_url(&mockito::server_url());
        let result = client
            .get_tokens("solana", None, None, Some(1), None, None, None)
            .await;
        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert_eq!(tokens.data.len(), 1);
        assert_eq!(
            tokens.data[0].address,
            "So11111111111111111111111111111111111111112"
        );
    }

    #[tokio::test]
    async fn test_search_tokens() {
        let _m = mock("GET", "/solana/tokens/search?q=sol")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "data": [],
                    "meta": {
                        "next": null,
                        "previous": null
                    }
                }"#,
            )
            .create();

        let client = OrcaClient::with_base_url(&mockito::server_url());
        let result = client.search_tokens("solana", "sol").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_token() {
        let _m = mock(
            "GET",
            "/solana/tokens/So11111111111111111111111111111111111111112",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
                "data": [],
                "meta": {
                    "next": null,
                    "previous": null
                }
            }"#,
        )
        .create();

        let client = OrcaClient::with_base_url(&mockito::server_url());
        let result = client
            .get_token("solana", "So11111111111111111111111111111111111111112")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_lock_info() {
        let _m = mock(
            "GET",
            "/solana/lock/Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"[
                {
                    "lockedPercentage": "0.7",
                    "name": "Whirlpool-Lock"
                }
            ]"#,
        )
        .create();

        let client = OrcaClient::with_base_url(&mockito::server_url());
        let result = client
            .get_lock_info(
                "solana",
                "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE",
            )
            .await;
        assert!(result.is_ok());
        let lock_info = result.unwrap();
        assert_eq!(lock_info.len(), 1);
        assert_eq!(lock_info[0].name, "Whirlpool-Lock");
    }

    #[tokio::test]
    async fn test_get_pools() {
        let _m = mock("GET", "/solana/pools?")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "data": [],
                    "meta": {
                        "next": null,
                        "previous": null
                    }
                }"#,
            )
            .create();
        let client = OrcaClient::with_base_url(&mockito::server_url());
        let params = GetPoolsParams::default();
        let result = client.get_pools("solana", params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_pools() {
        let _m = mock("GET", "/solana/pools/search?q=sol")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "data": [],
                    "meta": {
                        "next": null,
                        "previous": null
                    }
                }"#,
            )
            .create();
        let client = OrcaClient::with_base_url(&mockito::server_url());
        let params = SearchPoolsParams {
            q: "sol",
            ..Default::default()
        };
        let result = client.search_pools("solana", params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_pool() {
        let _m = mock(
            "GET",
            "/solana/pools/Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
                "data": [],
                "meta": {
                    "next": null,
                    "previous": null
                }
            }"#,
        )
        .create();
        let client = OrcaClient::with_base_url(&mockito::server_url());
        let result = client
            .get_pool("solana", "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE")
            .await;
        assert!(result.is_ok());
    }
}