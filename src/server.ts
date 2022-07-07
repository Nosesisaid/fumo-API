import {PORT} from './config/env';
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