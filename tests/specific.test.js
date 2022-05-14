const build = require("../dist/App.js").default;
const {test} = require("tap");

test('call `/fumo/id` route', async (t) => {
    t.plan(3)
    const app = build();
    const id = '6128c5578caf0cf9a83f73e8';
    const response = await app.inject({
        method: 'GET',
        url:'/fumo/'+id
    });

    t.teardown(() => app.close())

    const content = JSON.parse(response.payload);
    t.equal(response.statusCode, 200, 'status code is 200');
    t.ok(typeof content === 'object', 'response is an object');
    t.ok(content._id === id, 'response has the correct id');
  
  })
