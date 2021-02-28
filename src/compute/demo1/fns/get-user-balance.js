async function getUserBalance(ctx, db) {
  const balance = await db.get(`user::${ctx.urlParma}`);
  return { 
    user: ctx.urlParma,
    balance: Number(balance)
  };
}

module.exports = getUserBalance;
