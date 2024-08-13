// @generated automatically by Diesel CLI.

diesel::table! {
    assignments (id) {
        id -> Integer,
        name -> Text,
        assessment_type -> Text,
        unit -> Text,
        submitted -> Integer,
    }
}
