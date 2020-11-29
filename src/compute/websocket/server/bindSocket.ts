import * as stream from 'stream';
import Handler from './handler';
import runFns from './runFns';

export default function (socket: stream.Duplex) {
    let websocket = new Handler(socket);

    websocket.sendCheckPing();

    socket
        .on('data', (buffer: Buffer) => {
            websocket.getData(buffer, (socket, data) => {
                let sendMsg = "recieved message :" + data;

                runFns(JSON.parse(data));
                
                websocket.writeData(websocket.createData(sendMsg));
            });
        })
        .on('close', () => {
            console.log('socket close');
        })
        .on('end', () => {
            console.log('socket end');
        });
};
