use serde::{Serialize, Deserialize};

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
    pub partie: Option<i32>
}
