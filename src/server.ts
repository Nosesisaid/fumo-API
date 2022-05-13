import fastify from 'fastify';
import fastifyMongodb from '@fastify/mongodb';
import {PORT} from './config/env';

const App = fastify();
App.register(fastifyMongodb, {
	url: process.env.MONGO_URL
});

App.get('/', async (req, res) => {
	const fumos = await App.mongo.db?.collection('fumos').find({}).toArray();
    
	res.status(200).send(fumos);
});

App.get('/fumo/:id', async (req, res) => {

	const id = new App.mongo.ObjectId((req as any).params.id);
	const fumo = await (await App.mongo.db?.collection('fumos'))?.findOne({_id: id});

	res.status(200).send(fumo);
});

App.get('/random', async (req,res)=> {
	const fumos = await App.mongo.db?.collection('fumos').find({}).toArray();
	if (!fumos) return res.status(400).send('No fumo :( (server error)');
	const fumo = fumos[Math.floor(Math.random() * fumos?.length)];
    
	res.status(200).send(fumo);
});

App.get('/fumos', async(req, res) => {
	const fumos = await App.mongo.db?.collection('fumos').find({}).toArray();

	res.status(200).send(fumos);
});

App.listen(PORT, err => {
	if (err) throw err;
	console.log('Server listening on port ' + PORT);
});