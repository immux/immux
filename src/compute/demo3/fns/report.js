async function report(ctx, db) {
    const { codes = [] } = ctx.body;   
    console.log('report', codes) 
    return { codes };
}
  
module.exports = report;
  