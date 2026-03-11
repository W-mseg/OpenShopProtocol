pub mod manifest;
pub mod product;
pub mod validation;
pub mod well_known;

pub use manifest::ShopManifest;
pub use product::{Product, ProductListing, ProductPrice, ProductAsset, License};
pub use validation::{ValidationError, validate_shop_manifest, validate_product};
pub use well_known::WellKnownOsp;
