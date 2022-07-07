import fastify, { FastifyServerOptions } from 'fastify';
import fastifyMongodb from '@fastify/mongodb';
import './config/env';
import cors from '@fastify/cors';
import fastifySwagger from '@fastify/swagger';

async function build(opts: FastifyServerOptions = {}, isTest = false) {
	const App = fastify(opts);

	App.register(fastifyMongodb, {
		url: process.env.MONGO_URL,
	});
	App.register(cors, {
		origin: '*',
	});
	if (!isTest) {
		await App.register(fastifySwagger, {
			routePrefix: '/docs',
			swagger: {
				info: {
					title: 'Fumo-API',
					description: 'Rest API of images of fumos (japanese plushies).',
					version: '2.2.0',
					contact: {
						email: 'vic@nosesisaid.me',
						name: 'Nosesisaid',
						url: 'https://github.com/nosesisaid',
					},
					license: {
						name: 'MIT',
						url: 'https://opensource.org/licenses/MIT',
					},
				},
				externalDocs: {
					url: 'https://github.com/nosesisaid/fumo-api',
					description: 'Find more info here',
				},
				host: 'fumo-api.nosesisaid.me',
				schemes: ['https'],
				consumes: ['application/json'],
				produces: ['application/json'],
				tags: [
					{ name: 'images', description: 'image related end-points' },
				],
				definitions: {
					Fumo: {
						type: 'object',
						required: ['id', 'URL'],
						properties: {
							id: { type: 'string'},
							URL: { type: 'string' },
							caption: { type: 'string' },
							fumos: { type: 'array', } },
					},
				},
			
			},
			uiConfig: {
				deepLinking: false,
			},
			uiHooks: {
				onRequest: function (request, reply, next) {
					next();
				},
				preHandler: function (request, reply, next) {
					next();
				},
			},
			staticCSP: true,
			transformStaticCSP: (header) => header,
			exposeRoute: true,
		});
	}
	App.get('/', {
		schema: {
			description: 'Get the full list of fumos',
			tags: ['images'],
			response: {
				200: {
					type: 'array',
					items: {
						type: 'object',
						properties: {
							id: { type: 'string' },
							URL: { type: 'string' },
							caption: { type: 'string' },
							fumos: { type: 'array', },
						},
					},
				},
			},
		},
	},async (req, res) => {
		const fumos = await App.mongo.db?.collection('fumos').find({}).toArray();	
		res.status(200).send(fumos);
	});

	App.get('/fumo/:id',{
		schema: {
			description: 'Get a fumo by it\'s id',
			tags: ['images'],
			params: {
				type: 'object',
				properties: {
					id: { type: 'string', description: 'The id of the fumo' },
				},
			},
			response: {
				200: {
					type: 'object',
					properties: {
						id: { type: 'string' },
						URL: { type: 'string' },
						caption: { type: 'string' },
						fumos: { type: 'array', },
					},
				},
			},
		},
	}, async (req, res) => {
		//TODO: fix this
		const id = (req.params as { id: string }).id;
		
		const fumo = await (
			await App.mongo.db?.collection('fumos')
		)?.findOne({ _id: id });

		res.status(200).send(fumo);
	});

	App.get('/random', {
		schema: {
			description: 'Get a random fumo',
			tags: ['images'],
			response: {
				200: {
					type: 'object',
					properties: {
						id: { type: 'string' },
						URL: { type: 'string' },
						caption: { type: 'string' },
						fumos: { type: 'array', },
					},
				},
			},
		},
	}, async (req, res) => {
		const fumos = await App.mongo.db?.collection('fumos').find({}).toArray();
		if (!fumos) return res.status(400).send('No fumo :( (server error)');
		const fumo = fumos[Math.floor(Math.random() * fumos?.length)];

		res.status(200).send(fumo);
	});

	App.get('/fumos', {
		schema: {
			description: 'Get a list of fumos',
			tags: ['images'],
			response: {
				200: {
					type: 'array',
					items: {
						type: 'object',
						properties: {
							id: { type: 'string' },
							URL: { type: 'string' },
							caption: { type: 'string' },
							fumos: { type: 'array', },
						},
					},
				},
			},
		},
	}, async (req, res) => {
		const fumos = await App.mongo.db?.collection('fumos').find({}).toArray();

		res.status(200).send(fumos);
	});
	return App;
}
export default build;