import {FastifyInstance} from 'fastify';

interface app extends FastifyInstance {
    mongodb: {
        collection: (name: string) => {
            find: (query?: unknown) => Promise<unknown>;
            insertOne: (query: unknown) => Promise<unknown>;
            updateOne: (query: unknown) => Promise<unknown>;
            deleteOne: (query: unknown) => Promise<unknown>;
        }
    }
}
export {app};