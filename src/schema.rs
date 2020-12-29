table! {
    authors (id) {
        id -> Int4,
        name_prefix -> Nullable<Varchar>,
        name_first -> Nullable<Varchar>,
        name_von -> Nullable<Varchar>,
        name_last -> Nullable<Varchar>,
        name_suffix -> Nullable<Varchar>,
        birth -> Nullable<Int4>,
        death -> Nullable<Int4>,
        au_programme -> Bool,
        active -> Bool,
    }
}

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

table! {
    sessions (id) {
        id -> Int4,
        secret -> Varchar,
        sess_user -> Int4,
        acctime -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        display_name -> Varchar,
        email -> Varchar,
        password -> Text,
        active -> Bool,
        challenge_code -> Varchar,
        last_login -> Nullable<Date>,
        creation_time -> Timestamp,
    }
}

table! {
    works (id) {
        id -> Int4,
        title -> Text,
        containing_title -> Nullable<Text>,
        author -> Nullable<Int4>,
        active -> Bool,
    }
}

joinable!(sessions -> users (sess_user));
joinable!(works -> authors (author));

allow_tables_to_appear_in_same_query!(
    authors,
    prog_hlp,
    prog_notions,
    prog_reperes,
    sessions,
    users,
    works,
);
