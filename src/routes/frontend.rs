use askama::Template;
use axum::{
    Form, Router,
    extract::State,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;
use tokio::try_join;

use crate::{
    app::AppState,
    auth::user::{UnauthenticatedUser, User},
    error::AppError,
    models::{Asset, CurrencyComparison, OwnedAsset, PortfolioSummary},
    repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login_page).post(login))
        .route("/logout", get(logout))
        .route("/assets", get(assets).post(purchase_asset))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

async fn login_page() -> Result<Html<String>, AppError> {
    let html = LoginPage.render()?;
    Ok(Html(html))
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(
    repository: Repository,
    jar: CookieJar,
    Form(request): Form<LoginForm>,
) -> Result<impl IntoResponse, AppError> {
    let unauth_user = UnauthenticatedUser::new(request.username, request.password);
    let user = match unauth_user.authenticate(&repository).await {
        Ok(user) => user,
        Err(AppError::UserDoesNotExist) => unauth_user.register(&repository).await?,
        Err(other_err) => return Err(other_err),
    };
    let token = user.auth_token()?;
    let cookie = Cookie::build(("token", token)).http_only(true);

    Ok((jar.add(cookie), Redirect::to("/")))
}

pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    (jar.remove("token"), Redirect::to("/login"))
}

async fn index(maybe_user: Option<User>) -> Result<Redirect, AppError> {
    match maybe_user {
        Some(_) => Ok(Redirect::to("/assets")),
        None => Ok(Redirect::to("/login")),
    }
}

#[derive(Template)]
#[template(path = "assets.html")]
pub struct AssetsPage {
    owned_assets: Vec<OwnedAsset>,
    available_assets: Vec<Asset>,
    summary: PortfolioSummary,
    comparison: CurrencyComparison,
    user: User,
}

pub async fn assets(
    State(state): State<AppState>,
    repository: Repository,
    user: User,
) -> Result<Html<String>, AppError> {
    let (owned_assets, available_assets) = try_join!(
        repository.list_owned_assets(user.id()),
        repository.list_assets()
    )?;
    let summary = PortfolioSummary::from_assets(&owned_assets);
    let comparison = CurrencyComparison::from_usd(summary.current_value, state.usd_brl_rate);

    let html = AssetsPage {
        owned_assets,
        available_assets,
        summary,
        comparison,
        user,
    }
    .render()?;

    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct PurchaseAssetForm {
    asset_id: i64,
    unit_value: f64,
    quantity: f64,
}

pub async fn purchase_asset(
    repository: Repository,
    user: User,
    Form(request): Form<PurchaseAssetForm>,
) -> Result<Redirect, AppError> {
    validate_purchase(&request)?;

    if repository
        .get_asset_by_id(request.asset_id)
        .await?
        .is_none()
    {
        return Err(AppError::AssetDoesNotExist);
    }

    repository
        .insert_owned_asset(
            user.id(),
            request.asset_id,
            request.quantity,
            request.unit_value,
        )
        .await?;

    Ok(Redirect::to("/assets"))
}

fn validate_purchase(request: &PurchaseAssetForm) -> Result<(), AppError> {
    if !request.quantity.is_finite() || request.quantity <= 0.0 {
        return Err(AppError::InvalidPurchase(
            "a quantidade deve ser maior que zero".to_string(),
        ));
    }

    if !request.unit_value.is_finite() || request.unit_value <= 0.0 {
        return Err(AppError::InvalidPurchase(
            "o preço unitário deve ser maior que zero".to_string(),
        ));
    }

    Ok(())
}

pub mod filters {
    use time::{
        OffsetDateTime, format_description::StaticFormatDescription, macros::format_description,
    };

    #[askama::filter_fn]
    pub fn human_datetime(
        datetime: &OffsetDateTime,
        _env: &dyn askama::Values,
    ) -> askama::Result<String> {
        const HUMAN_READABLE_FORMAT: StaticFormatDescription =
            format_description!(version = 2, "[year]-[month]-[day] [hour]:[minute]");

        datetime
            .format(HUMAN_READABLE_FORMAT)
            .map_err(askama::Error::custom)
    }

    #[askama::filter_fn]
    pub fn usd(value: &f64, _env: &dyn askama::Values) -> askama::Result<String> {
        Ok(format!("US$ {value:.2}"))
    }

    #[askama::filter_fn]
    pub fn brl(value: &f64, _env: &dyn askama::Values) -> askama::Result<String> {
        Ok(format!("R$ {value:.2}"))
    }

    #[askama::filter_fn]
    pub fn decimal(value: &f64, _env: &dyn askama::Values) -> askama::Result<String> {
        let formatted = format!("{value:.4}");
        Ok(formatted
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn purchase(asset_id: i64, unit_value: f64, quantity: f64) -> PurchaseAssetForm {
        PurchaseAssetForm {
            asset_id,
            unit_value,
            quantity,
        }
    }

    #[test]
    fn accepts_valid_purchase() {
        assert!(validate_purchase(&purchase(1, 10.0, 0.5)).is_ok());
    }

    #[test]
    fn rejects_non_positive_quantity() {
        assert!(validate_purchase(&purchase(1, 10.0, 0.0)).is_err());
        assert!(validate_purchase(&purchase(1, 10.0, -1.0)).is_err());
    }

    #[test]
    fn rejects_non_positive_unit_value() {
        assert!(validate_purchase(&purchase(1, 0.0, 1.0)).is_err());
        assert!(validate_purchase(&purchase(1, -10.0, 1.0)).is_err());
    }
}
