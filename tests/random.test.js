const build = require("../dist/App.js").default;
const {test} = require("tap");
test('call `/random` route', async (t) => {
    t.plan(2)
    const app = build();
    const response = await app.inject({
        method: 'GET',
        url:'/random'
    });

    t.teardown(() => app.close())

    const content = JSON.parse(response.payload);
    t.equal(response.statusCode, 200, 'status code is 200');
    t.ok(typeof content === 'object', 'response is an object');
  
  })
