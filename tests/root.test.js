const build = require("../dist/App.js").default;
const { test } = require("tap");

test('call `/` route', async (t) => {
    t.plan(2);

    const app = await build();
    const response = await app.inject({
        method: 'GET',
        url:'/'
    });

    t.teardown(() => app.close());

    t.equal(response.statusCode, 200, 'status code is 200');
    t.ok(Array.isArray(JSON.parse(response.payload)), 'response is an array');

})

