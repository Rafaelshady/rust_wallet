use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use time::OffsetDateTime;

#[derive(Serialize, Clone)]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
}

pub struct UserRecord {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurchaseHistory {
    #[serde(with = "time::serde::iso8601")]
    pub bought_at: OffsetDateTime,
    pub bought_for: f64,
    pub quantity_bought: f64,
    pub value_delta: f64,
}

#[derive(Serialize)]
pub struct OwnedAsset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
    pub invested_value: f64,
    pub current_value: f64,
    pub value_delta: f64,
    pub quantity_owned: f64,
    pub purchase_history: Json<Vec<PurchaseHistory>>,
}

#[derive(Serialize, Default)]
pub struct PortfolioSummary {
    pub asset_count: usize,
    pub total_invested: f64,
    pub current_value: f64,
    pub total_change: f64,
}

impl PortfolioSummary {
    pub fn from_assets(assets: &[OwnedAsset]) -> Self {
        Self {
            asset_count: assets.len(),
            total_invested: assets.iter().map(|asset| asset.invested_value).sum(),
            current_value: assets.iter().map(|asset| asset.current_value).sum(),
            total_change: assets.iter().map(|asset| asset.value_delta).sum(),
        }
    }
}

#[derive(Serialize)]
pub struct CurrencyComparison {
    pub value_usd: f64,
    pub value_brl: f64,
    pub usd_brl_rate: f64,
}

impl CurrencyComparison {
    pub fn from_usd(value_usd: f64, usd_brl_rate: f64) -> Self {
        Self {
            value_usd,
            value_brl: value_usd * usd_brl_rate,
            usd_brl_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CurrencyComparison;

    #[test]
    fn converts_usd_value_to_brl() {
        let comparison = CurrencyComparison::from_usd(100.0, 5.5);

        assert_eq!(comparison.value_usd, 100.0);
        assert_eq!(comparison.value_brl, 550.0);
    }
}
