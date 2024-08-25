// @generated automatically by Diesel CLI.

diesel::table! {
    mosaic_design (id) {
        id -> Integer,
        name -> Text,
        filename -> Text,
        width_pixels -> Integer,
        height_pixels -> Integer,
        pixels -> Text,
    }
}
