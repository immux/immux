const user = require(`${requirePath}/app/fns/user`);
const info = require(`${requirePath}/app/fns/info`);
const register = require(`${requirePath}/app/fns/register`);
const { findRouter } = require(`${requirePath}/app/fns/common`);

const routers = {
    'GET /user': user,
    'GET /info/:id': info,
    'POST /register': register,
}

async function main(ctx, db) {
    const method = ctx.content.request.method;
    const path = ctx.provider.path;

    const routeKey = findRouter(method, path, routers)
    return routers[routeKey](ctx, db);  
}