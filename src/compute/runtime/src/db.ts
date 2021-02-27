import IORedis = require('ioredis');

export default new IORedis({
    db: 1
});