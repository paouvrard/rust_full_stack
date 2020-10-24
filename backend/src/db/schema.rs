table! {
    task (id) {
        id -> Integer,
        title -> Text,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    task,
    tasks,
);
