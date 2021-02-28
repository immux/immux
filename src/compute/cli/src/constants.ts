import chalk from 'chalk';
import * as os from 'os';
import path from 'path';

export const PREFIX = chalk.blue('[Compute CLI]');

export const INI_FILENAME = 'compute.ini';

// export const INI_FILEPATH = path.join(__dirname, INI_FILENAME);

export const PUBLIC_PEM_REGEXP = /^-----EMAIL\s+([^\n]+)\n(-----BEGIN\s+RSA\s+PUBLIC\s+KEY-----\n[^-]+\n-----END\s+RSA\s+PUBLIC\s+KEY-----)$/;

export const computeEnv = process.env.COMPUTE_ENV || 'release';

export const isDevelopment = computeEnv === 'development';

// prettier-ignore
export const apiOrigin = (() => {
  switch (computeEnv) {
    case 'development': {
      return 'http://localhost:3003';
    }

    default: {
      return 'https://comptest.immux.dev/api';
    }
  }
})();

export const publicPemFilename = (() => {
  switch (computeEnv) {
    case 'development': {
      return '.compute/compute_dev_rsa.pub';
    }

    default: {
      return '.compute/compute_rsa.pub';
    }
  }
})();

export const publicPemPath = path.join(os.homedir(), publicPemFilename);
