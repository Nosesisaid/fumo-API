# fumo API 


[![Tests](https://github.com/Nosesisaid/fumo-API/actions/workflows/tests.yml/badge.svg)](https://github.com/Nosesisaid/fumo-API/actions/workflows/tests.yml)


"API" of images of fumos (japanese Tohou plushies) using Rust.


![logo](https://repository-images.githubusercontent.com/395606928/753b9fdd-b978-4b74-841e-f3973daf9129)
## Documentation
REST API documentation is yet to be done. This project consists of 3 crates: `fumo-db, fumo-rest-api, fumo-bot` and two cloudflare workers: `fumo-rest-api` & `fumo-web`.


- The fumo-db is a library crate which fumo-bot and fumo-rest-api use to access and update the postgres database
- The fumo-rest-api is the main and original API but read only because I am not confident enough in myself to be able to filter a form open to the internet
- The fumo-bot is a Discord bot that works as a client of the "API" and allows you to add new entries to the DB through a text-message flow and a slash-command flow.
- fumo-web is a basic simple and ugly website made in svelte that will be the gallery of the fumo-API

Each has a readme.txt explaining how it works and specific information about it's dev environment.

## Contributing and running it locally
You can contribute with images to the fumo-API through the [official Discord server](https://discord.gg/3df68Hg6jF). Sorry for using Discord but this was the best option. 

if you would like something to be different or have any suggestion, please [open an issue](https://github.com/nosesisaid/fumo-api/issues/new).

To see how to start contributing or set up dev-environment, please read the CONTRIBUTING.md