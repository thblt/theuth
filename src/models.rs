use crate::schema::texts;

#[derive(Queryable)]
pub struct Texte {
    pub id: i32,
    pub slug: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="texts"]
pub struct NewTexte<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
