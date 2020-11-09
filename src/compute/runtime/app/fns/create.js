const { base64urlEscape } = require(`${requirePath}/app/fns/common`);

async function getBearerToken(authorization) {
  const parts = authorization.split(' ');
  return parts.length === 2 && parts[0] === 'Bearer' ? parts[1] : null;
}

async function base64urlUnescape(str) {
  str += new Array(5 - str.length % 4).join('=');
  return str.replace(/\-/g, '+').replace(/_/g, '/');
}

async function jsonTokenSign(content, secret) {
  let result = crypto.createHmac('sha256',secret).update(content).digest('base64');
  return base64urlEscape(result);
}

async function jsonTokenDecode(token, secret) {
  let [header, content, sign] = token.split('.');
  let newSign = await jsonTokenSign([header, content].join('.'), secret);
  
  if (sign === newSign) {
    const result = await base64urlUnescape(content)
    return Buffer.from(result, 'base64').toString();
  }
}

async function create(ctx, db) {
  const { code = '' } = ctx.body;
  const { authorization } = ctx.request.header;

  // debugger
  // const token = getBearerToken(authorization);

  const token = authorization.split(' ')[1];

  const secret = 'temp';
  const decodeToken = await jsonTokenDecode(token, secret);

  if (!decodeToken) {
      ctx.throw('invalid authorization');
  }
    
  const { user } = JSON.parse(decodeToken);

  await db.rpush(`code::${user}`, code)
  
  return {
    user,
    code,
  }
}

module.exports = create