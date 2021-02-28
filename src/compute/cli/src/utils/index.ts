import * as path from 'path';

export function getConfig() {
    const configDir = getAppPath() + '/config';

    const configDef = configDir + '/config';

    const confDef = require(configDef);

    return confDef
}

export function getAppPath() {
    return process.cwd()
}