const user = require(`${requirePath}/user`);
const userBalance = require(`${requirePath}/user-balance`);
const userBuy = require(`${requirePath}/user-buy`);
const getUserBalance = require(`${requirePath}/get-user-balance`);

const { findRouter } = require(`${requirePath}/common`);

const routers = {
  'POST /machine/:id': user,
  'GET /machine/:id/balance': getUserBalance,
  'POST /machine/:id/balance': userBalance,
  'POST /machine/:id/balance/buy': userBuy,
};

async function main(ctx, db) {
  const method = ctx.content.request.method;
  const path = ctx.provider.pathname;

  const { urlParma, routeKey } = findRouter(method, path, routers);
  ctx.urlParma = urlParma;
  return routers[routeKey](ctx, client);
}
