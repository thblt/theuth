table! {
    authors (id) {
        id -> Nullable<Integer>,
        slug -> Text,
        name_pre -> Nullable<Text>,
        name_first -> Nullable<Text>,
        name_von -> Nullable<Text>,
        name_last -> Nullable<Text>,
        name_suff -> Nullable<Text>,
        birth -> Nullable<Integer>,
        death -> Nullable<Integer>,
        au_programme -> Nullable<Integer>,
    }
}

table! {
    notions_philo (id) {
        id -> Nullable<Integer>,
        slug -> Text,
        name -> Text,
        le_name -> Text,
        techno -> Nullable<Integer>,
    }
}

table! {
    parties_hlp (id) {
        id -> Nullable<Integer>,
        slug -> Text,
        semestre -> Integer,
        partie -> Integer,
        name -> Text,
    }
}

table! {
    reperes_philo (id) {
        id -> Nullable<Integer>,
        slug -> Text,
        name -> Text,
    }
}

table! {
    sources (id) {
        id -> Nullable<Integer>,
        author -> Nullable<Integer>,
        title -> Text,
        containing_title -> Nullable<Text>,
    }
}

table! {
    texts (id) {
        id -> Nullable<Integer>,
        slug -> Text,
        title -> Text,
        body -> Text,
        forced_author -> Nullable<Integer>,
    }
}

table! {
    texts_notions (link_id) {
        link_id -> Nullable<Integer>,
        text_id -> Nullable<Integer>,
        notion_id -> Nullable<Integer>,
    }
}

joinable!(sources -> authors (author));
joinable!(texts -> authors (forced_author));
joinable!(texts_notions -> notions_philo (notion_id));
joinable!(texts_notions -> texts (text_id));

allow_tables_to_appear_in_same_query!(
    authors,
    notions_philo,
    parties_hlp,
    reperes_philo,
    sources,
    texts,
    texts_notions,
);
