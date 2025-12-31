# CONTRIBUTING

This project uses Rust and a cloudflare worker. To run the complete project you need: 
1. `rust and cargo`(developed using 1.87.0)
1. `diesel_cli` (for setting up the database [Installation docs](https://diesel.rs/guides/getting-started.html#installing-diesel-cli))
1. `libpq` (for the postgres database connections. Install it using your distro's repos)
1. `wrangler` (to publish the cloudflare worker. [Installation docs](https://developers.cloudflare.com/workers/wrangler/install-and-update/)).
1. Rust target `wasm32-unknown-unknown` (to be able to compile for web assembly)

To get started, run `cargo build` on the repository root. This will build all the rust code and check if you have `libpq` installed. 


To use any of the crate you must first setup the fumo-db. Read it's `readme.txt` for more info.

