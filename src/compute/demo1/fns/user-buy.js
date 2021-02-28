async function userBuy(ctx, db) {
  let balance = await db.get(`user::${ctx.urlParma}`);
  if ( balance > 0 ) { balance-- };
  await db.set(`user::${ctx.urlParma}`, balance);

  return { 
    user: ctx.urlParma,
    balance: Number(balance)
  };
}

module.exports = userBuy;
