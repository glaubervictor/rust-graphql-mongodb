use juniper::{graphql_object, EmptySubscription, GraphQLObject, RootNode};
use mongodb::{options::ClientOptions, sync::Client};

use crate::{
    models::cargo::{Cargo, CreateCargo},
    repositories::cargo::CargoRepository
};

#[derive(Clone, GraphQLObject)]
pub struct User {
    id: i32,
    name: String,
}

#[derive(Clone)]
pub struct Context {
    cargo_repository: CargoRepository,
}

impl Context {
    pub fn new() -> Context {
        let client_options =
            ClientOptions::parse("mongodb://localhost:27017").unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("mydb");
        let collection = db.collection::<Cargo>("cargos");
        let cargo_repository = CargoRepository::new(collection);
        Context { cargo_repository }
    }
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    pub fn api_version() -> &'static str {
        "1.0"
    }

    pub fn get_cargo_by_id(context: &Context, id: String) -> Option<Cargo> {
        context.cargo_repository.get_by_id(id)
    }

    pub fn get_cargo_all(context: &Context) -> Vec<Cargo> {
        context.cargo_repository.get_all()
    }

}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    pub fn create_cargo(context: &Context, input: CreateCargo) -> Cargo {
        context.cargo_repository.create(input)
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}
