async function addCode(ctx, db) {
    const { codes = [] } = ctx.body;
    codes.forEach(async (code) => {
        let number = await db.get(`scanner::${code}`);
        await db.set(`scanner::${code}`, number ? ++number : 1);
    });

    const postData = ctx.querystring.stringify({
        codes
    });

    // const postData = codes

    console.log('postData', postData)

    const options = {
        hostname: '8.134.88.89',
        port: 80,
        path: `/vm/demo/report`,
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
            'Content-Length': Buffer.byteLength(postData)
        }
    };
      
    const callback = (response) => {
        let str = '';
      
        response.on('data', (chunk) => {
            str += chunk;
        });
      
        response.on('end', () => {
            console.log(str);
        });
    }

    const req = ctx.http.request(options, callback);

    req.on('error', (e) => {
        console.error(`error: ${e.message}`);
    });

    req.write(postData);
    req.end();
    
    return { numbers: codes.length};
}
  
module.exports = addCode;