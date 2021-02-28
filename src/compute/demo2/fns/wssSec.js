async function wssSec(ctx, db) {
    console.log(wssSec, ctx.body)
    
    const { msg = '' } = ctx.body;
    
    return {
        fn: 'wssSec',
        msg,
    }
}

module.exports = wssSec