async function codeList(ctx, db) {
    let codes = [];

    const stream = await db.scanStream({
        match: "scanner:*",
        count: 100,
    });

    return new ctx.Promise((resolve, reject) => {
        stream.on("data", (resultKeys) => {
            for (let i = 0; i < resultKeys.length; i++) {
                codes.push(resultKeys[i].slice(9));
            }
        });
    
        stream.on("end", () => {
            resolve({ codes });
        });
    })
}
  
module.exports = codeList;