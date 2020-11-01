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
  const result = await scryptAsync(password, salt, 64);
  return result.toString("base64");
}

async function register(ctx, db) {
  const { password = "", user = "" } = ctx.body;

  // !!! node.js vm module problem, so sad
  // const passwordSalt = crypto.randomBytes(32).toString("base64");
  // const passwordHash = await hashPassword(password, passwordSalt);

  return {
    user,
    password,
  };
}

module.exports = register;
