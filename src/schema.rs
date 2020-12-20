table! {
    authors (id) {
        id -> Int4,
        slug -> Varchar,
        name_pre -> Nullable<Varchar>,
        name_first -> Nullable<Varchar>,
        name_von -> Nullable<Varchar>,
        name_last -> Nullable<Varchar>,
        name_suff -> Nullable<Varchar>,
        birth -> Nullable<Int4>,
        death -> Nullable<Int4>,
        official -> Nullable<Bool>,
    }
}

table! {
    sources (id) {
        id -> Int4,
        author -> Int4,
        title -> Text,
        date -> Nullable<Int4>,
    }
}

table! {
    texts (id) {
        id -> Int4,
        slug -> Varchar,
        title -> Varchar,
        intro -> Nullable<Text>,
        body -> Text,
        pseudo_author -> Nullable<Int4>,
        tsource -> Int4,
        translator -> Nullable<Int4>,
        publisher -> Nullable<Text>,
        date -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        display_name -> Nullable<Text>,
        password -> Text,
        password_is_temporary -> Nullable<Bool>,
    }
}

joinable!(sources -> authors (author));

allow_tables_to_appear_in_same_query!(
    authors,
    sources,
    texts,
    users,
);
