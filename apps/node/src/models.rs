use sqlx::FromRow;

/// Row in the shop table
#[derive(Debug, Clone, FromRow)]
pub struct ShopRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub email: Option<String>,
    pub lang: Option<String>,
    pub currency: Option<String>,
    pub logo_url: Option<String>,
    pub categories: Option<String>,
    pub tags: Option<String>,
    pub links: Option<String>,
}

/// Row in the products table
#[derive(Debug, Clone, FromRow)]
pub struct ProductRow {
    pub id: Option<String>,
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
    pub assets: Option<String>,
    pub categories: Option<String>,
    pub tags: Option<String>,
    pub version: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
