const user = require(`${requirePath}/user`);
const info = require(`${requirePath}/info`);
const wssFirst = require(`${requirePath}/wssFirst`);
const wssSec = require(`${requirePath}/wssSec`);
const { findRouter } = require(`${requirePath}/common`);

const routers = {
  "GET /user": user,
  "GET /info/:id": info,
  'POST /wssFirst': wssFirst,
  'POST /wssSec': wssSec,
};

async function main(ctx, db) {
  const method = ctx.content.request.method;
  const path = ctx.provider.path;

  const routeKey = findRouter(method, path, routers);
  return routers[routeKey](ctx, db);
}
