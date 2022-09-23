use bson::doc;
use mongodb::sync::Collection;
use uuid::Uuid;

use crate::models::cargo::{Cargo, CreateCargo};

#[derive(Clone)]
pub struct CargoRepository {
    collection: mongodb::sync::Collection<Cargo>,
}

impl CargoRepository {
    pub fn new(collection: Collection<Cargo>) -> CargoRepository {
        CargoRepository { collection }
    }

    pub fn create(&self, input: CreateCargo) -> Cargo {
        let cargo = Cargo {
            id: Uuid::new_v4().to_string(),
            codigo: input.codigo,
            nome: input.nome,
        };
        self.collection.insert_one(&cargo, None).unwrap();
        cargo
    }

    pub fn get_by_id(&self, id: String) -> Option<Cargo> {
        let filter = doc! { "id": id.to_string() };
        self.collection.find_one(filter, None).unwrap()
    }

    pub fn get_all(&self) -> Vec<Cargo> {
        let cursor = match self.collection.find(None, None) {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };
        //let cursor = self.collection.find(None, None).unwrap();
        let mut results: Vec<Cargo> = Vec::new();
        for doc in cursor {
            results.push(doc.unwrap());
        }
        results
    }
}
