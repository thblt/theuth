table! {
    prog_hlp (id) {
        id -> Int4,
        slug -> Text,
        name -> Text,
        semestre -> Int4,
        partie -> Nullable<Int4>,
    }
}

table! {
    prog_notions (id) {
        id -> Int4,
        slug -> Text,
        name -> Text,
        le_name -> Text,
        techno -> Bool,
    }
}

table! {
    prog_reperes (id) {
        id -> Int4,
        slug -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    prog_hlp,
    prog_notions,
    prog_reperes,
);
