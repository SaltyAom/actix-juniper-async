use crate::schema::Product;

lazy_static! {
    static ref PRODUCT: Vec<Product> =
        serde_json::from_str(include_str!("../static/product.json")).unwrap();
}

#[inline(always)]
pub fn get_products() -> Vec<Product> {
    PRODUCT.to_owned()
}
