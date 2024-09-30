CREATE TABLE sqlite_sequence(name,seq);
CREATE TABLE puzzle
(
    id   integer not null
        constraint puzzle_pk
            primary key autoincrement,
    name TEXT    not null
);
CREATE TABLE IF NOT EXISTS "user"
(
    id          integer not null
        constraint user_pk
            primary key autoincrement,
    sub         TEXT    not null,
    full_name   TEXT    not null,
    given_name  TEXT    not null,
    family_name TEXT    not null,
    created_at  integer not null,
    login_at    integer not null
);
CREATE TABLE competition
(
    id         integer not null
        constraint competition_pk
            primary key autoincrement,
    created_at integer not null,
    name       TEXT    not null
);
CREATE TABLE IF NOT EXISTS "scramble"
(
    id           integer not null
        constraint scramble_pk
            primary key autoincrement,
    scramble     TEXT    not null,
    puzzle       integer not null
        constraint scramble_puzzle_id_fk
            references puzzle,
    generated_at integer not null
);
CREATE TABLE _config
(
    competition   integer not null
        constraint _config_competition_id_fk
            references competition,
    cg_link       TEXT    not null,
    mosaic_design integer not null
        constraint _config_mosaic_design_id_fk
            references mosaic_design
);
CREATE TABLE IF NOT EXISTS "timed_solve"
(
    id           integer not null
        constraint timed_solve_pk
            primary key autoincrement,
    scramble     integer not null
        constraint timed_solve_scramble_id_fk
            references scramble,
    user         integer not null
        constraint timed_solve_user_id_fk
            references user,
    competition  integer not null
        constraint timed_solve_competition_id_fk
            references competition,
    time_ms      integer not null,
    penalty      TEXT,
    completed_at integer not null
);
CREATE TABLE role
(
    id   integer not null
        constraint role_pk
            primary key autoincrement,
    name TEXT    not null
);
CREATE TABLE user_role
(
    id   integer not null
        constraint user_role_pk
            primary key autoincrement,
    user integer not null
        constraint user_role_user_id_fk
            references user,
    role integer not null
        constraint user_role_role_id_fk
            references role
);
CREATE TABLE error
(
    id          integer not null
        constraint error_pk
            primary key autoincrement,
    time        TEXT    not null,
    code        integer not null,
    user        integer
        constraint error_user_id_fk
            references user,
    message     TEXT    not null,
    stack_trace TEXT    not null
);
CREATE TABLE IF NOT EXISTS "mosaic_tile"
(
    id          integer not null
        constraint mosaic_tile_pk
            primary key autoincrement,
    row         integer not null,
    col         integer not null,
    assigned_to integer
);
CREATE TABLE IF NOT EXISTS "mosaic_design"
(
    id            INTEGER            not null
        primary key autoincrement,
    name          VARCHAR            not null,
    width_pixels  INTEGER default 30 not null,
    height_pixels INTEGER default 30 not null,
    pixels        TEXT               not null
);
