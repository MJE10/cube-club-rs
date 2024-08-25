use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::mosaic_design)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MosaicDesign {
    pub id: i32,
    pub name: String,
    pub filename: String,
    pub width_pixels: i32,
    pub height_pixels: i32,
    pub pixels: String,
}
