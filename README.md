# fumo API 

Rest API of images of fumos (japanese plushies) using typescript and fastify.


## Issues
If you find a repeated fumo please create an issue

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
