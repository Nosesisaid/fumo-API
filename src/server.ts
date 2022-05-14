import {PORT} from './config/env';
import build from './App';

const server = build();

server.listen(PORT, err => {
	if (err) throw err;
	console.log('Server listening on port ' + PORT);
});