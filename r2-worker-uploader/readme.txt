
~~This is a simple worker that gets a discord proxy url, downloads it and uploads it to the r2 bucket. ~~


This is no longer a worker that downloads from the proxy url, since discord rate limits the requestes made by cloudflare workers.

I did not implement this on the fumo-bot or fumo-api binaries because r2 just adds a lot of unnecesary dependencies and complexity.