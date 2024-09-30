`cube-club-rs` is a Rust app for manages meetings for Cube Club at RIT.
It can help with tracking mosaics progress, scrambles,
timed solve leaderboards, and attendance.

# Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [sqlite3](https://www.sqlite.org/)
- [tnoodle-cli](https://github.com/SpeedcuberOSS/tnoodle-cli) (must be on PATH)

# Running the application

There are several steps required to run the app:

1. Setting up the database
2. Configuration files
3. Running via `cargo run`

## Setting up the database

1. Create an empty file in the repository root called `cube_club.db`
2. Import the schema by running `sql/schema.sql` against the database
    - `sqlite3 cube_club.db < sql/schema.sql`
3. Import default data by running `sql/setup.sql` against the database
    - `sqlite3 cube_club.db < sql/setup.sql`

## Configuration files

1. Create a file named `.env` in the repository root with this content:

```
DATABASE_URL=sqlite://cube_club.db

ROCKET_SECRET_KEY={{RANDOM_64DIGIT_NUMBER}}

ROCKET_OAUTH={google={client_id="{{GOOGLE_CLIENT_ID}}",client_secret="{{GOOGLE_CLIENT_SECRET}}"}}
```

2. Replace ROCKET_SECRET_KEY

Replace {{RANDOM_64DIGIT_NUMBER}} with a cryptographically randomly generated 64-digit number. This is used to store
client session information, so it should be consistent across all runs.

3. (Optional) Replace google client information

If you wish to use Google account authentication, replace {{GOOGLE_CLIENT_ID}} and {{GOOGLE_CLIENT_SECRET}} with the ID
and secret from your Google developer console.

## Running the app

Start the app in development mode with this command:

```bash
cargo run
```

Then go to [localhost:8000](localhost:8000) in a browser.

In development mode, any client may bypass authentication and login as user {{USER}} by visiting:
`localhost:8000/admin/iam_user/{{USER}}`

# Exporting the database

Follow these instructions if changes have been made to the database.

First, export the schema:

```bash
sqlite3 cube_club.db '.schema' > sql/schema.sql
```

Then, edit `sql/setup.sql` with any additional default data required for the app.

