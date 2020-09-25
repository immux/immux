import { ConnectionOptions } from 'mongoose';
import { RedisOptions } from 'ioredis';

export interface Dictionary<T> {
  [index: string]: T;
}

/**
 * {@link https://stackoverflow.com/a/48710483/4662191}
 */
export interface ErrnoException extends Error {
  errno?: number;
  code?: string;
  path?: string;
  syscall?: string;
  stack?: string;
}

export interface ComputeConfig {
  git: {
    origin: string;
    client_id: string;
    client_secret: string;
    api: string;
  };

  mongo: {
    uris: string;
    options: ConnectionOptions;
  };

  port: number;

  /**
   * token-secret
   */
  secret: string;
}

export type NextKoaMiddleware = (err?: any) => Promise<any>;

export interface AccountPems {
  /**
   * Private key hash
   */
  hash: string;

  email: string;

  publicPem: string;

  privatePem: string;

  createAt: number;
}

export interface FileNodeInfo {
  children: FileNodeInfo[];
  pNode: string;
  title: string;
  key: string;
  extname?: string;
  detail?: string;
  isLeaf?: Boolean;
}

export interface jsonTokenInfo {
  email: string;
  expiresIn: number;
}

