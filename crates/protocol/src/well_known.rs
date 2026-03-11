/// Served at `/.well-known/osp`
///
/// This is the entry point for any OSP-compatible crawler or client.
/// It points to the shop manifest and the product listing endpoint.
///
/// Example:
/// ```json
/// {
///   "osp_version": "1",
///   "shop_url": "https://example.com/shop.json",
///   "products_url": "https://example.com/products.json"
/// }
/// ```
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellKnownOsp {
    /// Protocol version. Current: "1"
    pub osp_version: String,

    /// Absolute URL to the shop manifest (`shop.json`)
    pub shop_url: Url,

    /// Absolute URL to the product listing (`products.json`)
    pub products_url: Url,
}

impl WellKnownOsp {
    pub fn new(base_url: &Url) -> Self {
        Self {
            osp_version: "1".to_string(),
            shop_url: base_url.join("/shop.json").expect("valid URL"),
            products_url: base_url.join("/products.json").expect("valid URL"),
        }
    }
}
