const user = require(`${requirePath}/user`);
const info = require(`${requirePath}/info`);
const { findRouter } = require(`${requirePath}/common`);

const routers = {
  "GET /user": user,
  "GET /info/:id": info,
};

async function main(ctx, db) {
  const method = ctx.content.request.method;
  const path = ctx.provider.path;

  const routeKey = findRouter(method, path, routers);
  return routers[routeKey](ctx, db);
}
