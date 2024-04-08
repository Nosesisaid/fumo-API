# fumo API 
# The project is currently down
> [!WARNING]  
> Official CDN is not currently working, since my education azure credit expired, the images are not aviable anymore. Therefore I guess the API is useless.


> [!IMPORTANT]  
> Please if you are willing to use this project for literally anything, let me know and I will try to make it usable again, you can create an Issue on this repo or [write to me](https://x.com/vicjajsalu2) Do it, for real. if you see this lmk.


[![Tests](https://github.com/Nosesisaid/fumo-API/actions/workflows/tests.yml/badge.svg)](https://github.com/Nosesisaid/fumo-API/actions/workflows/tests.yml)


Rest API of images of fumos (japanese plushies) using typescript and fastify.


![logo](https://repository-images.githubusercontent.com/395606928/753b9fdd-b978-4b74-841e-f3973daf9129)
## Documentation
Documentation available at [fumo-api.nosesisaid.com/docs](https://fumo-api.nosesisaid.com/docs)
## Contributing
if you would like something to be different or have any suggestion, please [open an issue](https://github.com/nosesisaid/fumo-api/issues/new).


## Running it locally 
### Using node 
1. Clone the repo `git clone https://github.com/nosesisaid/fumo-api.git`
1. Install dependencies `yarn`
1. Copy `.env.example` to `.env`
1. Fill the `.env` file with your config 
1. Build the project `yarn build`
1. Run the project `yarn start`
### Using docker
1. Pull the image `docker pull ghcr.io/nosesisaid/fumo-api:2.0.0`
1. Run the image `docker run -e MONGO_URL=<DatabaseUrl> -p <port>:3000 -d ghcr.io/nosesisaid/fumo-api:2.0.0`
