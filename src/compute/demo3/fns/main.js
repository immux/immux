const addCode = require(`${requirePath}/addCode`);
const report = require(`${requirePath}/report`);
const codeList = require(`${requirePath}/codeList`);
const info = require(`${requirePath}/info`);

const { findRouter } = require(`${requirePath}/common`);

const routers = {
  'POST /addCode': addCode,
  'POST /report': report,
  'GET /codeList': codeList,
  'GET /info': info,
};

async function main(ctx, db) {
  const method = ctx.content.request.method;
  const path = ctx.provider.pathname;
  const { urlParma, routeKey } = findRouter(method, path, routers);

  ctx.urlParma = urlParma;
  
  // todo node.js vm module problem, so sad
  ctx.querystring = querystring;
  ctx.http = http;
  ctx.Promise = Promise;

  return routers[routeKey](ctx, client);
}
