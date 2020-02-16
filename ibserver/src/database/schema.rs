table! {
    count (id) {
        hit_count -> Int4,
        id -> Int4,
        post_count -> Int4,
    }
}

table! {
    post_tags (id) {
        id -> Int4,
        tag_id -> Int4,
        post_id -> Int4,
    }
}

table! {
    posts (post_id) {
        post_id -> Int4,
        creation_date -> Date,
        hash -> Bpchar,
        original_name -> Varchar,
        height -> Int4,
        width -> Int4,
        ext -> Varchar,
        score -> Int4,
        directory -> Bpchar,
        ip -> Varchar,
    }
}

table! {
    tags (tag_id) {
        tag_id -> Int4,
        name -> Varchar,
        count -> Int4,
    }
}

joinable!(post_tags -> posts (post_id));
joinable!(post_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    count,
    post_tags,
    posts,
    tags,
);
