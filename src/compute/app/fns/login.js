const { base64urlEscape } = require(`./common`);

async function scryptAsync(password, salt, keylen) {
  return new Promise((resolve, reject) => {
    crypto.scrypt(password, salt, keylen, (error, derivedKey) => {
      if (error) {
        reject(error);
      } else {
        resolve(derivedKey);
      }
    });
  });
}

async function hashPassword(password, salt) {
  return (await scryptAsync(password, salt, 64)).toString("base64");
}

async function toBase64(content) {
  return base64urlEscape(
    Buffer.from(JSON.stringify(content)).toString("base64")
  );
}

async function jsonTokenSign(content, secret) {
  let result = crypto
    .createHmac("sha256", secret)
    .update(content)
    .digest("base64");
  return base64urlEscape(result);
}

async function jsonTokenEncode(info, secret) {
  let header = await toBase64({ typ: "JWT", alg: "HS256" });
  let content = await toBase64(info);
  let sign = await jsonTokenSign([header, content].join("."), secret);

  return [header, content, sign].join(".");
}

async function login(ctx, db) {
  const { password = "", user = "" } = ctx.body;
  const { salt, hash } = await db.hgetall(`user::${user}`);

  const passwordHash = await hashPassword(password, salt);

  if (hash !== passwordHash) {
    return "error";
  }

  // const expiresIn = new Date()
  const expiresIn = +moment().add(24, "hours");
  const secret = "temp";

  const accessToken = await jsonTokenEncode(
    {
      user,
      expiresIn,
    },
    secret
  );

  return {
    user,
    accessToken,
    expiresIn,
  };
}

module.exports = login;
