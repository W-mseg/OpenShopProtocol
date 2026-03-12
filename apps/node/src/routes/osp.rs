use axum::{extract::{Query, State}, Json};
use osp_protocol::{
    WellKnownOsp, ShopManifest,
    product::{ProductListing, Product, ProductPrice, ProductType, PriceModel, License, ProductAsset, AssetType},
};
use serde::Deserialize;
use url::Url;

use crate::{error::AppResult, AppState};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// GET /.well-known/osp
pub async fn well_known(State(state): State<AppState>) -> AppResult<Json<WellKnownOsp>> {
    let base: Url = state.config.public_url.parse().unwrap();
    Ok(Json(WellKnownOsp::new(&base)))
}

/// GET /shop.json
pub async fn shop_manifest(State(state): State<AppState>) -> AppResult<Json<ShopManifest>> {
    let row = sqlx::query_as!(
        crate::models::ShopRow,
        "SELECT * FROM shop WHERE id = 1"
    )
    .fetch_one(&state.db)
    .await?;

    let base: Url = state.config.public_url.parse().unwrap();

    let manifest = ShopManifest {
        osp_version: "1".into(),
        url: base,
        name: row.name,
        description: row.description,
        owner: row.owner,
        email: row.email,
        lang: row.lang,
        currency: row.currency,
        logo_url: row.logo_url.and_then(|u: String| u.parse::<url::Url>().ok()),
        categories: row.categories
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default(),
        tags: row.tags
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default(),
        links: row.links.as_deref().and_then(|s| serde_json::from_str(s).ok()),
    };

    Ok(Json(manifest))
}

/// GET /products.json?page=1&per_page=20
pub async fn products_listing(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> AppResult<Json<ProductListing>> {
    let page     = pagination.page.unwrap_or(1).max(1);
    let per_page = pagination.per_page.unwrap_or(50).clamp(1, 100);
    let offset   = (page - 1) * per_page;

    let total: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM products")
        .fetch_one(&state.db)
        .await? as i64;

    let rows: Vec<crate::models::ProductRow> = sqlx::query_as!(
        crate::models::ProductRow,
        "SELECT * FROM products ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        per_page, offset
    )
    .fetch_all(&state.db)
    .await?;

    let base: Url = state.config.public_url.parse().unwrap();

    // Build next_url if there are more pages
    let next_url = if offset + per_page < total {
        let next_page = page + 1;
        base.join(&format!("/products.json?page={next_page}&per_page={per_page}")).ok()
    } else {
        None
    };

    let products: Vec<Product> = rows.into_iter()
        .map(|r| row_to_product(r, &base))
        .collect();

    Ok(Json(ProductListing {
        osp_version: "1".into(),
        shop_url: base,
        total: total as u64,
        page: Some(page as u32),
        next_url,
        products,
    }))
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn row_to_product(r: crate::models::ProductRow, base: &Url) -> Product {
    let product_url = r.url.as_deref().unwrap_or("").parse().unwrap_or_else(|_| {
        base.join(&format!("/products/{}", r.id.as_deref().unwrap_or("unknown"))).unwrap()
    });

    let product_type = match r.product_type.as_deref().unwrap_or("download") {
        "subscription" => ProductType::Subscription,
        "tool"         => ProductType::Tool,
        "content"      => ProductType::Content,
        "physical"     => ProductType::Physical,
        _              => ProductType::Download,
    };

    let price_model = match r.price_model.as_deref().unwrap_or("free") {
        "fixed"             => PriceModel::Fixed,
        "pay_what_you_want" => PriceModel::PayWhatYouWant,
        "subscription"      => PriceModel::Subscription,
        _                   => PriceModel::Free,
    };

    let license = if r.license_spdx.is_some() || r.license_name.is_some() {
        Some(License {
            spdx: r.license_spdx,
            name: r.license_name,
            url:  r.license_url.and_then(|u| u.parse().ok()),
        })
    } else {
        None
    };

    let assets: Vec<ProductAsset> = r.assets
        .as_deref()
        .and_then(|s| serde_json::from_str::<Vec<serde_json::Value>>(s).ok())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|v| {
            let url = v["url"].as_str()?.parse().ok()?;
            let asset_type = match v["asset_type"].as_str().unwrap_or("other") {
                "screenshot" => AssetType::Screenshot,
                "preview"    => AssetType::Preview,
                "video"      => AssetType::Video,
                "demo"       => AssetType::Demo,
                _            => AssetType::Other,
            };
            Some(ProductAsset { asset_type, url, alt: v["alt"].as_str().map(str::to_string) })
        })
        .collect();

    Product {
        id:               r.id.unwrap_or_default(),
        name:             r.name.unwrap_or_default(),
        description:      r.description,
        long_description: r.long_description,
        url:              product_url,
        product_type,
        price: ProductPrice {
            model:    price_model,
            amount:   r.price_amount.map(|a| a as u64),
            currency: r.price_currency,
        },
        license,
        cover_url:  r.cover_url.and_then(|u| u.parse().ok()),
        assets,
        categories: r.categories.as_deref().and_then(|s| serde_json::from_str(s).ok()).unwrap_or_default(),
        tags:       r.tags.as_deref().and_then(|s| serde_json::from_str(s).ok()).unwrap_or_default(),
        created_at: r.created_at,
        updated_at: r.updated_at,
        version:    r.version,
    }
}
