use crate::models::usuario::UsuarioSlim;
use anyhow::Result;
use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Clone)]
pub struct DecodedToken {
    pub jwt: Option<Claims>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // issuer
    pub iss: String,
    // subject
    pub sub: String,
    // issued at
    pub iat: i64,
    // expiry
    pub exp: i64,
    // user email
    pub email: String,
    // user role
    pub role: String,
}

// struct to get converted to token and back
impl Claims {
    pub(crate) fn new(
        usuario_slim: &UsuarioSlim,
        issuer: String,
        auth_duration_in_hour: u16,
    ) -> Self {
        let UsuarioSlim {
            id, email, role, ..
        } = usuario_slim;

        let iat = Local::now();
        let exp = iat + Duration::hours(i64::from(auth_duration_in_hour));

        Claims {
            iss: issuer,
            sub: id.to_string(),
            email: email.clone(),
            role: role.clone(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Token {
    pub bearer: Option<String>,
}

impl TryFrom<Claims> for UsuarioSlim {
    type Error = anyhow::Error;

    fn try_from(claims: Claims) -> Result<Self> {
        let Claims {
            email, sub, role, ..
        } = claims;

        Ok(UsuarioSlim {
            id: Uuid::parse_str(&sub)?.to_string(),
            email,
            role,
        })
    }
}
