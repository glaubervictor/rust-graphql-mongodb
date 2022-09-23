use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, GraphQLObject)]
pub struct Cargo {
    pub id: String,
    pub codigo: i32,
    pub nome: String,
}

#[derive(GraphQLInputObject)]
pub struct CreateCargo {
    pub codigo: i32,
    pub nome: String,
}
