import 'reflect-metadata';
import '@/mongo';

import Koa = require('koa');

import { createKoaServer } from 'routing-controllers';
import controllers from '@/controllers';
import middlewares from '@/middlewares';

const app: Koa = createKoaServer({
  defaultErrorHandler: false,
  classTransformer: false,
  validation: false,
  controllers,
  middlewares
});

export default app;
