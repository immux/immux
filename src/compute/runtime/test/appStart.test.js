

const Frame = require("../dist/src/core").Frame;


let assert = require("chai").assert;
const app = new Frame;
const server = app.run();

describe("app start", () => {
    it('get', async () => {
        const j = await app.curl('http://127.0.0.1:3001/example');
        assert.equal(j.body, 'example get');
    })
    it('post', async () => {
        const j = await app.post('http://127.0.0.1:3001/example', { foo: 'good' });
        assert.equal(j.body, '{"foo":"good"}');
    })
    it('register', async () => {
        const j = await app.post('http://127.0.0.1:3001/vm/anchor/register', {user: 'root', password: '123456'});
        assert.equal(j.body, '{"user":"root","password":"123456"}');
    })
})

setTimeout(() => {
    server.close(function () { console.log('Server closed!'); });
}, 1000)

