use crate::{ShopManifest, Product};
use crate::product::PriceModel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("osp_version must be '1', got '{0}'")]
    UnsupportedVersion(String),

    #[error("field '{0}' is required")]
    MissingField(&'static str),

    #[error("field '{field}' is too long (max {max} chars, got {got})")]
    TooLong { field: &'static str, max: usize, got: usize },

    #[error("price.amount is required for model '{0}'")]
    MissingAmount(String),

    #[error("price.currency is required when amount is set")]
    MissingCurrency,
}

pub fn validate_shop_manifest(manifest: &ShopManifest) -> Result<(), ValidationError> {
    if manifest.osp_version != "1" {
        return Err(ValidationError::UnsupportedVersion(manifest.osp_version.clone()));
    }

    if manifest.name.is_empty() {
        return Err(ValidationError::MissingField("name"));
    }

    if let Some(desc) = &manifest.description {
        if desc.len() > 1000 {
            return Err(ValidationError::TooLong { field: "description", max: 1000, got: desc.len() });
        }
    }

    Ok(())
}

pub fn validate_product(product: &Product) -> Result<(), ValidationError> {
    if product.id.is_empty() {
        return Err(ValidationError::MissingField("id"));
    }

    if product.name.is_empty() {
        return Err(ValidationError::MissingField("name"));
    }

    let price = &product.price;

    match price.model {
        PriceModel::Fixed | PriceModel::Subscription => {
            if price.amount.is_none() {
                return Err(ValidationError::MissingAmount(format!("{:?}", price.model)));
            }
        }
        PriceModel::PayWhatYouWant | PriceModel::Free => {}
    }

    if price.amount.is_some() && price.currency.is_none() {
        return Err(ValidationError::MissingCurrency);
    }

    if let Some(desc) = &product.description {
        if desc.len() > 1000 {
            return Err(ValidationError::TooLong { field: "description", max: 1000, got: desc.len() });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ProductPrice, ProductType};
    use url::Url;

    fn make_product(model: PriceModel, amount: Option<u64>, currency: Option<&str>) -> Product {
        Product {
            id: "test-product".into(),
            name: "Test Product".into(),
            description: None,
            long_description: None,
            url: Url::parse("https://example.com/products/test").unwrap(),
            product_type: ProductType::Download,
            price: ProductPrice {
                model,
                amount,
                currency: currency.map(str::to_string),
            },
            license: None,
            cover_url: None,
            assets: vec![],
            categories: vec![],
            tags: vec![],
            created_at: None,
            updated_at: None,
            version: None,
        }
    }

    #[test]
    fn free_product_is_valid() {
        let p = make_product(PriceModel::Free, None, None);
        assert!(validate_product(&p).is_ok());
    }

    #[test]
    fn fixed_product_requires_amount() {
        let p = make_product(PriceModel::Fixed, None, None);
        assert!(matches!(validate_product(&p), Err(ValidationError::MissingAmount(_))));
    }

    #[test]
    fn fixed_product_requires_currency() {
        let p = make_product(PriceModel::Fixed, Some(1000), None);
        assert!(matches!(validate_product(&p), Err(ValidationError::MissingCurrency)));
    }

    #[test]
    fn fixed_product_valid() {
        let p = make_product(PriceModel::Fixed, Some(1000), Some("EUR"));
        assert!(validate_product(&p).is_ok());
    }
}
