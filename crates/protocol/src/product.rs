/// Core product types for OSP.
///
/// `ProductListing` is served at `/products.json` and contains a list of `Product`.
/// Each product is a self-contained description of a digital good.
use serde::{Deserialize, Serialize};
use url::Url;

// ── Listing ──────────────────────────────────────────────────────────────────

/// Served at `/products.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListing {
    /// Protocol version. Must be "1"
    pub osp_version: String,

    /// Canonical shop URL (must match shop.json)
    pub shop_url: Url,

    /// Total number of products (may exceed this page)
    pub total: u64,

    /// Pagination: current page (1-indexed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// URL to the next page, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_url: Option<Url>,

    /// The products on this page
    pub products: Vec<Product>,
}

// ── Product ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    /// Stable unique identifier within this shop (slug or UUID)
    pub id: String,

    /// Display name
    pub name: String,

    /// Short description (max 280 chars recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Long-form description, may contain Markdown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,

    /// Canonical URL of the product page
    pub url: Url,

    /// Product type
    #[serde(rename = "type")]
    pub product_type: ProductType,

    /// Pricing model
    pub price: ProductPrice,

    /// License under which this product is distributed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,

    /// Cover image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<Url>,

    /// Additional media (screenshots, previews, demo videos)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<ProductAsset>,

    /// Category slugs (e.g. "fonts", "templates", "tools")
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,

    /// Freeform tags
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    /// ISO 8601 creation date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// ISO 8601 last-updated date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    /// Current version string (semver or freeform)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

// ── Product type ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    /// Downloadable file (font, template, preset, zip…)
    Download,
    /// SaaS / subscription access
    Subscription,
    /// One-time access to a hosted tool
    Tool,
    /// Written content (ebook, guide, course)
    Content,
    /// Physical product (edge case, allowed by the spec)
    Physical,
    /// Any other type not covered above
    Other,
}

// ── Price ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPrice {
    pub model: PriceModel,

    /// Amount in minor units (e.g. cents). Required for `fixed` and `min` models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,

    /// ISO 4217 currency code (e.g. "EUR", "USD")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PriceModel {
    /// Fixed price
    Fixed,
    /// Pay what you want (optionally with a minimum)
    PayWhatYouWant,
    /// Free
    Free,
    /// Recurring subscription
    Subscription,
}

// ── Asset ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductAsset {
    pub asset_type: AssetType,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    Screenshot,
    Preview,
    Video,
    Demo,
    Other,
}

// ── License ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    /// SPDX identifier if applicable (e.g. "MIT", "Apache-2.0", "CC-BY-4.0")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx: Option<String>,

    /// Human-readable license name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// URL to the full license text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}
