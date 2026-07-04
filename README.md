# fumo API 


[![Tests](https://github.com/Nosesisaid/fumo-API/actions/workflows/tests.yml/badge.svg)](https://github.com/Nosesisaid/fumo-API/actions/workflows/tests.yml)


"API" of images of fumos (japanese Tohou plushies). Built with Rust.


![logo](https://repository-images.githubusercontent.com/395606928/753b9fdd-b978-4b74-841e-f3973daf9129)
## Documentation
REST API documentation is yet to be done. This project consists of 3 crates: `fumo-db, fumo-rest-api, fumo-bot` and two cloudflare workers: `r2-worker-uploader` & `fumo-web`.


- The fumo-db is a library crate which fumo-bot and fumo-rest-api use to access and manipulate the postgres database
- The fumo-rest-api is the main service, the API itself. Read only for vandalism prevention 
- The fumo-bot is a Discord bot that 
    1. Works as a client of the "API" 
    1. Enables users to submit entries to the fumo-db through a text-message flow and a slash-command flow inside the Nosesisaid Discord server.
- The fumo-web is a basic simple, basic and ugly website Built with svelte

Each service has a readme.txt in its directory explaining it more in depth and with specific information about its dev environment.


## Contributing and running it locally
You can contribute with images to the fumo-API through the [official Discord server](https://discord.gg/3df68Hg6jF). I'm not very fond of Discord but it's the best way to prevent abuse for now. 

if you would like something to be different or have any suggestion, please [open an issue](https://github.com/nosesisaid/fumo-api/issues/new).

To see how to start contributing or set up dev-environment, please read the CONTRIBUTING.md
