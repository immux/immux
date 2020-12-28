import * as crypto from 'crypto';
import * as http from 'http';
import * as stream from 'stream';

const cryptoKey = '258EAFA5-E914-47DA-95CA-C5AB0DC85B11';

// Calculate the handshake response accept-key
let challenge = (reqKey) => {
    reqKey += cryptoKey;
    reqKey = reqKey.replace(/\s/g,"");
    return crypto.createHash('sha1').update(reqKey).digest().toString('base64');
}

export default function(req: http.IncomingMessage, socket: stream.Duplex, head: Buffer) {
    let _headers = req.headers,
        _key = _headers['sec-websocket-key'],
        resHeaders = [],
        br = "\r\n";

    resHeaders.push(
        'HTTP/1.1 101 WebSocket Protocol Handshake is OK',
        'Upgrade: websocket',
        'Connection: Upgrade',
        'Sec-WebSocket-Origin: ' + _headers.origin,
        'Sec-WebSocket-Location: ws://' + _headers.host + req.url,
    );

    let resAccept = challenge(_key);
    resHeaders.push('Sec-WebSocket-Accept: '+ resAccept + br, head);
    socket.write(resHeaders.join(br), 'binary');
}