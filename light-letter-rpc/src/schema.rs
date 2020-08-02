table! {
    categories (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    config (key) {
        key -> Text,
        value -> Text,
    }
}

table! {
    post_tags (post, tag) {
        post -> Uuid,
        tag -> Text,
    }
}

table! {
    posts (id) {
        id -> Uuid,
        status -> Int4,
        timestamp -> Timestamp,
        url -> Nullable<Text>,
        title -> Text,
        #[sql_name = "abstract"]
        abstract_ -> Text,
        content -> Text,
        file -> Nullable<Text>,
        series -> Nullable<Uuid>,
        category -> Uuid,
        commentable -> Int4,
    }
}

table! {
    series (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Text,
        name -> Text,
        pwd -> Text,
        email -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

joinable!(post_tags -> posts (post));
joinable!(posts -> categories (category));
joinable!(posts -> series (series));

allow_tables_to_appear_in_same_query!(
    categories,
    config,
    post_tags,
    posts,
    series,
    users,
);
