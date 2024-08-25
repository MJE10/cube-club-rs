-- Your SQL goes here
CREATE TABLE mosaic_design
(
    id            INTEGER NOT NULL PRIMARY KEY,
    name          VARCHAR NOT NULL,
    filename      VARCHAR NOT NULL,
    width_pixels  INTEGER NOT NULL DEFAULT 30,
    height_pixels INTEGER NOT NULL DEFAULT 30,
    pixels        TEXT    NOT NULL
);