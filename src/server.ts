import 'dotenv/config';
const PORT = process.env.PORT || 3000;
import build from './App';
async function main() {
	
	const server = await build({
		logger: true
	});
	await server.ready();
	await server.swagger();
	server.listen(PORT,'0.0.0.0', err => {
		if (err) throw err;
		console.log('Server listening on port ' + PORT);
	});
}
main();