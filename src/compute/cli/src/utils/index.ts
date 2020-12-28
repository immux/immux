import * as path from 'path';

export function getConfig() {
    const configDir = getAppPath() + '/config';

    const configDef = configDir + '/config.default';
    const configEnv = configDir
        + (process.env.NODE_ENV === 'production' ? '/config.pro' : '/config.dev');
    
    const conf = require(configEnv).default;
    const confDef = require(configDef).default;

    return Object.assign({}, conf, confDef);
}

export function getAppPath() {
    return path.join(process.cwd(), '..', 'app')
}