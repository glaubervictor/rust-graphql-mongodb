use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;

use crate::util::passwords::{make_hash, make_salt};

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct Usuario {
    pub id: String,
    #[graphql(skip)]
    pub hash: Vec<u8>,
    #[graphql(skip)]
    pub salt: String,
    pub email: String,
    pub role: String,
    pub nome: String,
    pub criado_em: NaiveDateTime,
}

#[derive(Debug)]
pub struct CreateUsuario {
    pub id: String,
    pub hash: Vec<u8>,
    pub salt: String,
    pub email: String,
    pub role: String,
    pub nome: String,
    pub criado_em: NaiveDateTime,
}

#[derive(Debug, Deserialize, GraphQLInputObject)]
pub struct UsuarioData {
    pub nome: String,
    pub email: String,
    pub senha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
pub struct UsuarioSlim {
    pub id: String,
    pub email: String,
    pub role: String,
}

#[derive(Default, Clone, Shrinkwrap)]
pub struct UsuarioLogged(pub Option<UsuarioSlim>);

impl From<UsuarioSlim> for UsuarioLogged {
    fn from(usuario_slim: UsuarioSlim) -> Self {
        UsuarioLogged(Some(usuario_slim))
    }
}

impl From<UsuarioData> for CreateUsuario {
    fn from(usuario_data: UsuarioData) -> Self {
        let UsuarioData {
            nome, email, senha, ..
        } = usuario_data;

        let salt = make_salt();
        let hash = make_hash(&senha, &salt).to_vec();
        Self {
            id: Uuid::new_v4().to_string(),
            email,
            hash,
            criado_em: chrono::Local::now().naive_local(),
            salt,
            nome,
            role: "usuario".to_owned(),
        }
    }
}
