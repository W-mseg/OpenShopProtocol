use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::{AppError, AppResult}, AppState};

// ── Auth helper ───────────────────────────────────────────────────────────────

fn require_auth(headers: &HeaderMap, password: &str) -> AppResult<()> {
    let auth = headers
        .get("x-admin-password")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if auth != password {
        return Err(AppError::Unauthorized);
    }
    Ok(())
}

// ── Shop ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct UpdateShop {
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub email: Option<String>,
    pub lang: Option<String>,
    pub currency: Option<String>,
    pub logo_url: Option<String>,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub links: Option<serde_json::Value>,
}

/// GET /admin/shop
pub async fn get_shop(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> AppResult<Json<serde_json::Value>> {
    require_auth(&headers, &state.config.admin_password)?;

    let row = sqlx::query!("SELECT * FROM shop WHERE id = 1")
        .fetch_one(&state.db)
        .await?;

    Ok(Json(serde_json::json!({
        "name": row.name,
        "description": row.description,
        "owner": row.owner,
        "email": row.email,
        "lang": row.lang,
        "currency": row.currency,
        "logo_url": row.logo_url,
        "categories": row.categories.and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok()),
        "tags": row.tags.and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok()),
        "links": row.links.and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok()),
    })))
}

/// PATCH /admin/shop
pub async fn update_shop(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(body): Json<UpdateShop>,
) -> AppResult<StatusCode> {
    require_auth(&headers, &state.config.admin_password)?;

    sqlx::query!(
        r#"UPDATE shop SET
            name        = COALESCE($1, name),
            description = COALESCE($2, description),
            owner       = COALESCE($3, owner),
            email       = COALESCE($4, email),
            lang        = COALESCE($5, lang),
            currency    = COALESCE($6, currency),
            logo_url    = COALESCE($7, logo_url),
            categories  = COALESCE($8, categories),
            tags        = COALESCE($9, tags),
            links       = COALESCE($10, links)
        WHERE id = 1"#,
        body.name,
        body.description,
        body.owner,
        body.email,
        body.lang,
        body.currency,
        body.logo_url,
        body.categories.map(|v| serde_json::to_string(&v).unwrap()),
        body.tags.map(|v| serde_json::to_string(&v).unwrap()),
        body.links.map(|v| serde_json::to_string(&v).unwrap()),
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

// ── Products ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub url: String,
    pub product_type: Option<String>,
    pub price_model: String,
    pub price_amount: Option<i64>,
    pub price_currency: Option<String>,
    pub license_spdx: Option<String>,
    pub license_name: Option<String>,
    pub license_url: Option<String>,
    pub cover_url: Option<String>,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreatedProduct {
    pub id: String,
}

/// GET /admin/products
pub async fn list_products(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> AppResult<Json<serde_json::Value>> {
    require_auth(&headers, &state.config.admin_password)?;

    let rows = sqlx::query!("SELECT * FROM products ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await?;

    let products: Vec<serde_json::Value> = rows.into_iter().map(|r| serde_json::json!({
        "id": r.id,
        "name": r.name,
        "description": r.description,
        "url": r.url,
        "product_type": r.product_type,
        "price_model": r.price_model,
        "price_amount": r.price_amount,
        "price_currency": r.price_currency,
        "cover_url": r.cover_url,
        "categories": r.categories.and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok()),
        "tags": r.tags.and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok()),
        "version": r.version,
        "created_at": r.created_at,
        "updated_at": r.updated_at,
    })).collect();

    Ok(Json(serde_json::json!({ "products": products })))
}

/// POST /admin/products
pub async fn create_product(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(body): Json<CreateProduct>,
) -> AppResult<(StatusCode, Json<CreatedProduct>)> {
    require_auth(&headers, &state.config.admin_password)?;

    let id = Uuid::new_v4().to_string();
    let product_type = body.product_type.unwrap_or_else(|| "download".into());
    let categories = serde_json::to_string(&body.categories.unwrap_or_default()).unwrap();
    let tags = serde_json::to_string(&body.tags.unwrap_or_default()).unwrap();

    sqlx::query!(
        r#"INSERT INTO products
            (id, name, description, long_description, url, product_type,
             price_model, price_amount, price_currency,
             license_spdx, license_name, license_url,
             cover_url, categories, tags, version)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16)"#,
        id,
        body.name,
        body.description,
        body.long_description,
        body.url,
        product_type,
        body.price_model,
        body.price_amount,
        body.price_currency,
        body.license_spdx,
        body.license_name,
        body.license_url,
        body.cover_url,
        categories,
        tags,
        body.version,
    )
    .execute(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(CreatedProduct { id })))
}

/// DELETE /admin/products/:id
pub async fn delete_product(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    require_auth(&headers, &state.config.admin_password)?;

    let result = sqlx::query!("DELETE FROM products WHERE id = $1", id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
