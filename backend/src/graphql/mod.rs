mod mutation;
mod query;

use async_graphql::{EmptySubscription, Schema};

pub use mutation::MutationRoot;
pub use query::QueryRoot;

use crate::store::Db;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(db: Db) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish()
}
