/// Served at `/shop.json`
///
/// Describes the shop itself: identity, owner, contact, categories.
/// This is what crawlers and indexes read first after discovering a shop via `/.well-known/osp`.
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopManifest {
    /// Protocol version. Must be "1"
    pub osp_version: String,

    /// Canonical URL of the shop (no trailing slash)
    pub url: Url,

    /// Display name of the shop
    pub name: String,

    /// Short description (max 280 chars recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Owner or creator name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    /// Contact email (optional, public)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Shop language (BCP 47, e.g. "en", "fr")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    /// Currency used (ISO 4217, e.g. "EUR", "USD")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// URL to the shop logo/avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<Url>,

    /// List of product category slugs this shop covers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,

    /// Freeform tags
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    /// Social links and external profiles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<ShopLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastodon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<Url>,
}
