use axum::{extract::State, Json};
use osp_protocol::{
    WellKnownOsp,
    ShopManifest,
    product::{ProductListing, Product, ProductPrice, ProductType, PriceModel, License, ProductAsset, AssetType},
    manifest::ShopLinks,
};
use url::Url;

use crate::{error::AppResult, AppState};

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
        logo_url: row.logo_url.and_then(|u| u.parse().ok()),
        categories: row.categories
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default(),
        tags: row.tags
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default(),
        links: row.links.and_then(|s| serde_json::from_str(&s).ok()),
    };

    Ok(Json(manifest))
}

/// GET /products.json
pub async fn products_listing(State(state): State<AppState>) -> AppResult<Json<ProductListing>> {
    let rows = sqlx::query_as!(
        crate::models::ProductRow,
        "SELECT * FROM products ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await?;

    let total = rows.len() as u64;
    let base: Url = state.config.public_url.parse().unwrap();

    let products: Vec<Product> = rows.into_iter().map(|r| row_to_product(r, &base)).collect();

    Ok(Json(ProductListing {
        osp_version: "1".into(),
        shop_url: base,
        total,
        page: Some(1),
        next_url: None,
        products,
    }))
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn row_to_product(r: crate::models::ProductRow, base: &Url) -> Product {
    let product_url = r.url.parse().unwrap_or_else(|_| {
        base.join(&format!("/products/{}", r.id)).unwrap()
    });

    let product_type = match r.product_type.as_str() {
        "subscription" => ProductType::Subscription,
        "tool"         => ProductType::Tool,
        "content"      => ProductType::Content,
        "physical"     => ProductType::Physical,
        _              => ProductType::Download,
    };

    let price_model = match r.price_model.as_str() {
        "fixed"            => PriceModel::Fixed,
        "pay_what_you_want"=> PriceModel::PayWhatYouWant,
        "subscription"     => PriceModel::Subscription,
        _                  => PriceModel::Free,
    };

    let license = if r.license_spdx.is_some() || r.license_name.is_some() {
        Some(License {
            spdx: r.license_spdx,
            name: r.license_name,
            url: r.license_url.and_then(|u| u.parse().ok()),
        })
    } else {
        None
    };

    let assets: Vec<ProductAsset> = r.assets
        .and_then(|s| serde_json::from_str::<Vec<serde_json::Value>>(&s).ok())
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
            Some(ProductAsset {
                asset_type,
                url,
                alt: v["alt"].as_str().map(str::to_string),
            })
        })
        .collect();

    Product {
        id: r.id,
        name: r.name,
        description: r.description,
        long_description: r.long_description,
        url: product_url,
        product_type,
        price: ProductPrice {
            model: price_model,
            amount: r.price_amount.map(|a| a as u64),
            currency: r.price_currency,
        },
        license,
        cover_url: r.cover_url.and_then(|u| u.parse().ok()),
        assets,
        categories: r.categories
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default(),
        tags: r.tags
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default(),
        created_at: Some(r.created_at),
        updated_at: Some(r.updated_at),
        version: r.version,
    }
}
