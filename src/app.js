import fastify from "fastify";
import fastifyMongo from "fastify-mongodb";

const app = fastify();

app.register(fastifyMongo, {
    forceClose: true,
    url: process.env.MONGOURL
})

app.get("/", async (request, reply) => {
    reply
        .status(400)
        .send("Here are no fumos, random fumo in /random");
})

app.get("/random", async(request, reply) => {
    this.mongo.db.collection("fumos").find().toArray((err, fumos) => {
        const fumo = fumos[Math.floor(Math.random() * fumos.length)];
        if(err) {
            reply.status(500).send(err);
        }
        reply.status(200).send(fumo);
    })
})

app.get("/fumos", async(request, reply) => {
    this.mongo.db.collection("fumos").find().toArray((err, fumos) => {
        if(err) {
            reply.status(500).send(err)
        };
        reply.status(200).send(fumos);
    })
})

app.get("/fumos/:id", async(request, reply) => {
    const id = request.params["id"];
    this.mongo.db.collection("fumos").findOne({_id: id}, (err, fumo) => {
        if(err) {
            reply.status(500).send(err);
        }
        reply.status(200).send(fumo);
    })
})


export default app;