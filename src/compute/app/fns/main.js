const user = require(`./user`);
const info = require(`./info`);
const register = require(`./register`);
const { findRouter } = require(`./common`);

const routers = {
  "GET /user": user,
  "GET /info/:id": info,
  "POST /register": register,
};

async function main(ctx, db) {
  const method = ctx.content.request.method;
  const path = ctx.provider.path;

  const routeKey = findRouter(method, path, routers);
  return routers[routeKey](ctx, db);
}
