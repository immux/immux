async function userBalance(ctx, db) {
  const balance = ctx.provider.query.new_balance;
  await db.set(`user::${ctx.urlParma}`, balance);

  return { 
    user: ctx.urlParma,
    balance: Number(balance)
  };
}

module.exports = userBalance;
