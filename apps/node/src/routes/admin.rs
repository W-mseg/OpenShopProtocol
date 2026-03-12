use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::verify_password,
    error::{AppError, AppResult},
    validation::{max_len, require_str, validate_price, validate_url},
    AppState,
};

// ── Auth ──────────────────────────────────────────────────────────────────────

fn require_auth(headers: &HeaderMap, hash: &str) -> AppResult<()> {
    let password = headers
        .get("x-admin-password")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !verify_password(password, hash) {
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
    require_auth(&headers, &state.config.admin_password_hash)?;

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
        "categories": row.categories.as_deref().and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok()),
        "tags": row.tags.as_deref().and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok()),
        "links": row.links.as_deref().and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok()),
    })))
}

/// PATCH /admin/shop
pub async fn update_shop(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(body): Json<UpdateShop>,
) -> AppResult<StatusCode> {
    require_auth(&headers, &state.config.admin_password_hash)?;

    // Validate
    if let Some(ref name) = body.name {
        require_str("name", name)?;
        max_len("name", name, 120)?;
    }
    if let Some(ref desc) = body.description {
        max_len("description", desc, 1000)?;
    }
    if let Some(ref url) = body.logo_url {
        if !url.is_empty() { validate_url("logo_url", url)?; }
    }

    let categories = body.categories.map(|v| serde_json::to_string(&v).unwrap());
    let tags       = body.tags.map(|v| serde_json::to_string(&v).unwrap());
    let links      = body.links.map(|v| serde_json::to_string(&v).unwrap());

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
        categories,
        tags,
        links,
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

// ── Products ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

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

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub url: Option<String>,
    pub product_type: Option<String>,
    pub price_model: Option<String>,
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

/// GET /admin/products?page=1&per_page=20
pub async fn list_products(
    headers: HeaderMap,
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    require_auth(&headers, &state.config.admin_password_hash)?;

    let page     = pagination.page.unwrap_or(1).max(1);
    let per_page = pagination.per_page.unwrap_or(20).clamp(1, 100);
    let offset   = (page - 1) * per_page;

    let total: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM products")
        .fetch_one(&state.db)
        .await? as i64;

    let rows: Vec<_> = sqlx::query!(
        "SELECT * FROM products ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        per_page, offset
    )
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
        "categories": r.categories.as_deref().and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok()),
        "tags": r.tags.as_deref().and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok()),
        "version": r.version,
        "created_at": r.created_at,
        "updated_at": r.updated_at,
    })).collect();

    let total_pages = (total as f64 / per_page as f64).ceil() as i64;

    Ok(Json(serde_json::json!({
        "products": products,
        "total": total,
        "page": page,
        "per_page": per_page,
        "total_pages": total_pages,
    })))
}

/// POST /admin/products
pub async fn create_product(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(body): Json<CreateProduct>,
) -> AppResult<(StatusCode, Json<CreatedProduct>)> {
    require_auth(&headers, &state.config.admin_password_hash)?;

    // Validate
    require_str("name", &body.name)?;
    max_len("name", &body.name, 200)?;
    require_str("url", &body.url)?;
    validate_url("url", &body.url)?;
    if let Some(ref desc) = body.description {
        max_len("description", desc, 1000)?;
    }
    if let Some(ref url) = body.cover_url {
        if !url.is_empty() { validate_url("cover_url", url)?; }
    }
    validate_price(
        &body.price_model,
        body.price_amount,
        body.price_currency.as_deref(),
    )?;

    let id           = Uuid::new_v4().to_string();
    let product_type = body.product_type.unwrap_or_else(|| "download".into());
    let categories   = serde_json::to_string(&body.categories.unwrap_or_default()).unwrap();
    let tags         = serde_json::to_string(&body.tags.unwrap_or_default()).unwrap();

    sqlx::query!(
        r#"INSERT INTO products
            (id, name, description, long_description, url, product_type,
             price_model, price_amount, price_currency,
             license_spdx, license_name, license_url,
             cover_url, categories, tags, version)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16)"#,
        id, body.name, body.description, body.long_description,
        body.url, product_type, body.price_model,
        body.price_amount, body.price_currency,
        body.license_spdx, body.license_name, body.license_url,
        body.cover_url, categories, tags, body.version,
    )
    .execute(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(CreatedProduct { id })))
}

/// PATCH /admin/products/:id
pub async fn update_product(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateProduct>,
) -> AppResult<StatusCode> {
    require_auth(&headers, &state.config.admin_password_hash)?;

    // Check exists
    let exists = sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE id = $1", id)
        .fetch_one(&state.db)
        .await?;
    if exists == 0 {
        return Err(AppError::NotFound);
    }

    // Validate
    if let Some(ref name) = body.name {
        require_str("name", name)?;
        max_len("name", name, 200)?;
    }
    if let Some(ref url) = body.url {
        validate_url("url", url)?;
    }
    if let Some(ref desc) = body.description {
        max_len("description", desc, 1000)?;
    }
    if let Some(ref url) = body.cover_url {
        if !url.is_empty() { validate_url("cover_url", url)?; }
    }
    if body.price_model.is_some() || body.price_amount.is_some() {
        validate_price(
            body.price_model.as_deref().unwrap_or("free"),
            body.price_amount,
            body.price_currency.as_deref(),
        )?;
    }

    let categories = body.categories.map(|v| serde_json::to_string(&v).unwrap());
    let tags       = body.tags.map(|v| serde_json::to_string(&v).unwrap());

    sqlx::query!(
        r#"UPDATE products SET
            name            = COALESCE($1,  name),
            description     = COALESCE($2,  description),
            long_description= COALESCE($3,  long_description),
            url             = COALESCE($4,  url),
            product_type    = COALESCE($5,  product_type),
            price_model     = COALESCE($6,  price_model),
            price_amount    = COALESCE($7,  price_amount),
            price_currency  = COALESCE($8,  price_currency),
            license_spdx    = COALESCE($9,  license_spdx),
            license_name    = COALESCE($10, license_name),
            license_url     = COALESCE($11, license_url),
            cover_url       = COALESCE($12, cover_url),
            categories      = COALESCE($13, categories),
            tags            = COALESCE($14, tags),
            version         = COALESCE($15, version),
            updated_at      = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
        WHERE id = $16"#,
        body.name, body.description, body.long_description,
        body.url, body.product_type, body.price_model,
        body.price_amount, body.price_currency,
        body.license_spdx, body.license_name, body.license_url,
        body.cover_url, categories, tags, body.version,
        id,
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /admin/products/:id
pub async fn delete_product(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    require_auth(&headers, &state.config.admin_password_hash)?;

    let result: sqlx::sqlite::SqliteQueryResult =
        sqlx::query!("DELETE FROM products WHERE id = $1", id)
            .execute(&state.db)
            .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
