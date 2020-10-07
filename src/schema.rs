use juniper::{GraphQLObject, RootNode, EmptyMutation, EmptySubscription};
use serde::{Deserialize, Serialize};

use crate::data::get_products;

#[derive(Serialize, Deserialize, GraphQLObject, Clone)]
pub struct Product {
    pub name: String,
    pub kind: String,
    pub technical: String,
    pub color: String,
    pub storage: i32,
}

pub struct QueryRoot {}

#[juniper::graphql_object]
impl QueryRoot {
    async fn get_all_product() -> Vec<Product> {
        get_products()
    }

    async fn get_product_by(name: String) -> Vec<Product> {
        let search_key = name.to_lowercase();

        let filtered: Vec<Product> = get_products()
            .iter()
            .filter(move |&product| product.name.to_lowercase().contains(&search_key))
            .cloned()
            .collect();

        filtered
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
