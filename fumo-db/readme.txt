Library crate that is used by the fumo rest API and the fumo-bot.

Needs a postgres database to work.

# Database setup
Once you have a postgres database running, copy the `.env.example` of this directory to `.env` and fill it with you database url.

You will need the `diesel_cli` (https://diesel.rs/guides/getting-started)

Then, run `diesel database setup`. This will create the database and run all the necessary migrations.