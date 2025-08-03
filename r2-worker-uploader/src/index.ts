interface Env {
	AUTH_KEY_SECRET: string;
	FUMOS_BUCKET: R2Bucket;
}

interface ReqBody {
	image_proxy_url?:string;
}

const isAuthenticated = (request: Request, env: Env) =>{
	return request.headers.get("X-Custom-Auth-Key") === env.AUTH_KEY_SECRET;
}

export default {
	async fetch(request, env, ctx): Promise<Response> {
		
		const url = new URL(request.url);
		let key = url.pathname.slice(1);
		if (!isAuthenticated(request,env)) {
			return new Response("Forbidden", {status: 403})
		}
		if (key === "") {
			return new Response("Bad Request. You must provide a key", {status: 400})
		}


		switch (request.method){
			case "POST":
				const body: ReqBody = await request.json();
				if (!body.image_proxy_url) {
					return new Response("Bad Request. You must provide an image_proxy_url", {status: 400})
				}
				const imageBuff = await fetch(body.image_proxy_url);
				if (!imageBuff.ok || !imageBuff.body) {
					let text = await imageBuff.text()
					return new Response("Failed to fetch imaged. Got status "+imageBuff.status+ text,{status:502});
				}
				await env.FUMOS_BUCKET.put(key, imageBuff.body );

				return Response.json({key_in_bucket:key})
			default: 
			return new Response("Method not allowed", {status: 405})
		}
	},
} satisfies ExportedHandler<Env>;
