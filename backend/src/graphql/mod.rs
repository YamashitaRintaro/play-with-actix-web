mod mutation;
pub mod query;

use async_graphql::{EmptySubscription, Schema};
use mutation::MutationRoot;
use query::QueryRoot;

use crate::store::Db;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(db: Db) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish()
}
