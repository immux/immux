async function wssFirst(ctx, db) {
    console.log(wssFirst, ctx.body)
    
    const { msg = '' } = ctx.body;
    
    return {
        fn: 'wssFirst',
        msg,
    }
}

module.exports = wssFirst