use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct ProgNotion {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub le_name: String,
    pub techno: bool,
}

#[derive(Queryable, Serialize)]
pub struct ProgRepere {
    pub id: i32,
    pub slug: String,
    pub name: String,
}

#[derive(Queryable, Serialize)]
pub struct ProgHLP {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub semestre: i32,
    pub partie: Option<i32>,
}

#[derive(Queryable, Serialize)]
pub struct Author<'a> {
    pub id: i32,
    pub name_prefix: Option<&'a str>,
    pub name_first: Option<&'a str>,
    pub name_von: Option<&'a str>,
    pub name_last: Option<&'a str>,
    pub name_suffix: Option<&'a str>,
    pub birth: i32,
    pub death: i32,
    pub au_programme: bool,
}

use crate::schema::authors;

#[derive(Deserialize, Insertable)]
#[table_name = "authors"]
pub struct AuthorForm {
    pub name_prefix: Option<String>,
    pub name_first: Option<String>,
    pub name_von: Option<String>,
    pub name_last: Option<String>,
    pub name_suffix: Option<String>,
}

use crate::schema::users;

#[derive(Deserialize, Insertable, Queryable)]
#[table_name = "users"]
pub struct AuthRegisterForm {
    pub display_name: String,
    pub email: String,
    pub password: String
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct AuthLoginForm {
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct AuthActivationForm {
    pub id: i32,
    pub secret: String
}
