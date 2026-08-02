use crate::{
    app::AppState, auth::admin::Admin, error::AppError, models::Asset, repository::Repository,
};
use axum::{Json, Router, routing::get};
use serde::Deserialize;

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/assets",
        get(list_assets).post(create_asset).patch(update_asset),
    )
}

#[tracing::instrument(skip_all)]
async fn list_assets(repository: Repository) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repository.list_assets().await?;
    Ok(Json(assets))
}
#[derive(Deserialize)]
struct CreateAssetRequest {
    name: String,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
async fn create_asset(
    _admin: Admin,
    repository: Repository,
    Json(request): Json<CreateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    let new_asset = repository
        .create_asset(request.name, request.unit_value)
        .await?;

    Ok(Json(new_asset))
}

#[derive(Deserialize)]
struct UpdateAssetRequest {
    id: i64,
    name: Option<String>,
    unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
async fn update_asset(
    _admin: Admin,
    repository: Repository,
    Json(request): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    match repository
        .update_asset(request.id, request.name, request.unit_value)
        .await?
    {
        Some(updated_asset) => Ok(Json(updated_asset)),
        None => Err(AppError::AssetDoesNotExist),
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test]
    async fn test_create_asset(db: PgPool) {
        let request = CreateAssetRequest {
            name: "Cardano".to_string(),
            unit_value: 2.5,
        };
        let Json(new_asset) = create_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert!(new_asset.id > 0);
        assert_eq!(new_asset.name, "Cardano");
        assert_eq!(new_asset.unit_value, 2.5);
    }

    #[sqlx::test]
    async fn test_list_assets(db: PgPool) {
        let Json(assets) = list_assets(db.into()).await.expect("success");

        assert_eq!(assets.len(), 5);

        for expected_name in ["Bitcoin", "Ethereum", "Solana", "Dólar", "Real"] {
            assert!(
                assets.iter().any(|asset| asset.name == expected_name),
                "asset {expected_name} should be available"
            );
        }
    }

    #[sqlx::test]
    async fn test_update_asset(db: PgPool) {
        let request = UpdateAssetRequest {
            id: 1,
            name: Some("Bitcoin Plus".to_string()),
            unit_value: Some(20.0),
        };
        let Json(updated_asset) = update_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(updated_asset.id, 1);
        assert_eq!(updated_asset.name, "Bitcoin Plus");
        assert_eq!(updated_asset.unit_value, 20.0);
    }
}
