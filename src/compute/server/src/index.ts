require('module-alias').addAliases({ '@': __dirname });

import { ErrnoException } from '@/types';

import { createServer } from 'http';
import { debug } from '@/utils';
import config from './config';
import _ = require('lodash');
import app from '@/app';

/**
 * Create HTTP server.
 */
const server = createServer(app.callback());

/**
 * Listen on provided port, on all network interfaces.
 */
server.listen(config.port);

server.on('error', (error: ErrnoException) => {
  if (error.syscall !== 'listen') {
    throw error;
  }

  const bind = `Port: ${config.port}`;

  // handle specific listen errors with friendly messages
  switch (error.code) {
    case 'EACCES': {
      console.error(`${bind} requires elevated privileges`);
      process.exit(1);
      break;
    }

    case 'EADDRINUSE': {
      console.error(`${bind} is already in use`);
      process.exit(1);
      break;
    }

    default: {
      throw error;
    }
  }
});

server.on('listening', () => {
  const addr = server.address();
  const bind = _.isString(addr) ? `pipe ${addr}` : `port ${addr.port}`;

  debug.app(`Listening on ${bind}`);
});
