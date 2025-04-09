// This file will be auto-generated by diesel_cli after running migrations
// For now, we'll create a placeholder

// Example schema for users table:
/*
table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    payments (id) {
        id -> Int4,
        user_id -> Int4,
        amount -> Numeric,
        currency -> Varchar,
        status -> Varchar,
        payment_method -> Nullable<Varchar>,
        description -> Nullable<Text>,
        external_reference -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    installments (id) {
        id -> Int4,
        payment_id -> Int4,
        amount -> Numeric,
        due_date -> Date,
        paid_date -> Nullable<Date>,
        status -> Varchar,
        external_reference -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(installments -> payments (payment_id));
joinable!(payments -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    payments,
    installments,
);
*/