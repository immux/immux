import * as http from 'http';
import * as stream from 'stream';

import handshaking from './handshaking';
import bindSocketEvent from './bindSocket';

const port = 8082;

let server = http.createServer();

// HTTP upgrade
server.on('upgrade', (req: http.IncomingMessage, socket: stream.Duplex, head: Buffer) =>{
    // check
    if(req.headers.upgrade != 'websocket'){
        console.log('Not a WebSocket connection');
        socket.end();
    }
    
    // bind socket
    bindSocketEvent(socket);

    // handshake event
    try {
        handshaking(req, socket, head);

    } catch (error){
        console.log(error);
        socket.end();
    }
});

// close
server.on('close', () => {

});

server.listen(port, () => {
    console.log('sever is running!')
});