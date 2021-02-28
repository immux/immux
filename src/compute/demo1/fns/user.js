async function user(ctx, db) {
  await db.set(`user::${ctx.urlParma}`, 0);
  return { user: ctx.urlParma};
}

module.exports = user;
