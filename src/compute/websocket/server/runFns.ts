import * as http from 'http';
import * as querystring from 'querystring';

interface Message {
    fn: string,
    msg: string,
}

export default function runFns(data: Message) {
    const postData = querystring.stringify({
        msg: data.msg
    });

    const options = {
        hostname: '127.0.0.1',
        port: 3001,
        path: `/vm/demo/${data.fn}`,
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

    const req = http.request(options, callback);

    req.on('error', (e) => {
        console.error(`error: ${e.message}`);
    });

    req.write(postData);
    req.end();
}