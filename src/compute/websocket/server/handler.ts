import * as stream from 'stream';

interface HandlerState {
    index: number;
    remains? : number;
    opcode? : number;
    payloadData? : Buffer;
    payloadLength? : number;
    maskingKey?: Buffer;
}

export default class Handler {
    private socket: stream.Duplex;
    private state: HandlerState;
    private dataList: Buffer[];
    private pingTimes: number;
    private OPEN: Boolean;

    constructor(socket: stream.Duplex){
        this.socket = socket;
        this.state = { index : 0 };
        this.dataList = [];
        this.pingTimes = 0;
        this.OPEN = true;
    }

    // Parse the current frame state
    getState(data: Buffer) {
        let data01 = data[0].toString(2), data02 = data[1].toString(2);
        let fin = data01.slice(0, 1); // get fin
        let opcode = parseInt(data01.slice(4), 2); // Convert a binary string to decimal
        let dataIndex = 2; // initial data subscript
        let masked = data02.slice(0, 1);
        let payloadLength = parseInt(data02.slice(1), 2);
        let payloadData;
        let maskingKey;
        
        
        if(payloadLength == 126) {
            dataIndex += 2;
            payloadLength = data.readUInt16BE(2); //  Read 16 bits from left to right starting from 3 bytes (that is, two bytes 3, 4)
            dataIndex += 8; // The data length is the next 8 bytes
            payloadLength = data.readUInt32BE(2) + data.readUInt32BE(6); // First read 3, 4, 6, 7, and then read 8, 9, 10, 11
        }
    
        // Determine if there is masking key
        if(masked === "1") {
            maskingKey = data.slice(dataIndex, dataIndex + 4);
            dataIndex += 4; // Relocate data
            payloadData = data.slice(dataIndex);
        }
        let remains = this.state.remains || payloadLength; // Remaining data length
        remains = remains - payloadData.length; // How much length is left payload data
        
        Object.assign(this.state, {
            fin,
            opcode,
            masked,
            dataIndex,
            maskingKey,
            payloadData,
            payloadLength,
            remains
        });
    }

    // Collect all the data of this message
    getData(data, callback) {
        this.getState(data);

        // close
        if(this.state.opcode == 8) {
            this.OPEN = false;
            this.closeSocket();
            return;
        }

        // If it is a heartbeat pong, return a ping
        if(this.state.opcode == 10) {
            this.OPEN = true;
            this.pingTimes = 0;
            return;
        }

        this.dataList.push(this.state.payloadData);

        if(this.state.remains == 0){
            let buf = Buffer.concat(this.dataList, this.state.payloadLength);
            // Use maskingKey to parse all data
            let result = this.parseData(buf);
            callback(this.socket, result);
            this.resetState();
        }else{
            this.state.index++;
        }
    }

    // Analyze all data of this message
    parseData(allData, callback? : Function){
        let len = allData.length,
            i = 0;

        for(; i < len; i++){
            allData[i] = allData[i] ^ this.state.maskingKey[ i % 4 ];
        }

        if(this.state.opcode == 1) allData = allData.toString();

        return allData;
    }

    // Assemble the data frame
    createData(data){
        let dataType = Buffer.isBuffer(data);

        let dataBuf, // Binary data to be sent
            dataLength,
            dataIndex = 2; // The starting length of the data

        let frame;

        if (dataType) {
            dataBuf = data;
        } else {
            dataBuf = Buffer.from(data);
        }    
        dataLength = dataBuf.byteLength; 
        
        // Calculate the starting position of the payload data in the frame
        dataIndex = dataIndex + (dataLength > 65535 ? 8 : (dataLength > 125 ? 2 : 0));

        frame = Buffer.alloc(dataIndex + dataLength);

        frame[0] = parseInt('10000001', 2);

        if (dataLength > 65535){
            frame[1] = 127;
            frame.writeUInt32BE(0, 2); 
            frame.writeUInt32BE(dataLength, 6);
        } else if (dataLength > 125){
            frame[1] = 126;
            frame.writeUInt16BE(dataLength, 2);
        } else {
            frame[1] = dataLength;
        }

        // Data sent from the server to the client
        frame.write(dataBuf.toString(), dataIndex);

        return frame;
    }

    // Heartbeat check
    sendCheckPing(){
        let _this = this;
        let timer = setTimeout(() => {
            clearTimeout(timer);
            if (_this.pingTimes >= 3) {
                _this.closeSocket();
                return;
            }
            this.sendPing();
            _this.pingTimes++;
            _this.sendCheckPing();
        }, 5000);
    }

    sendPing() {
        let ping = Buffer.alloc(2);
        ping[0] = parseInt('10001001', 2);
        ping[1] = 0;
        this.writeData(ping);
    }

    closeSocket(){
        this.socket.end();
    }

    writeData(data){
        if(this.OPEN){
            this.socket.write(data);
        }
    }

    resetState() {
        this.dataList = [];
        this.state.index = 0;
    }    
}