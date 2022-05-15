import {PORT} from './config/env';
import build from './App';

const server = build({
	logger: true
});

server.listen(PORT,'0.0.0.0', err => {
	if (err) throw err;
	console.log('Server listening on port ' + PORT);
});