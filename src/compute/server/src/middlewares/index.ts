import { KoaMiddlewareInterface, Middleware } from 'routing-controllers';
import { NextKoaMiddleware } from '@/types';
import { Context } from 'koa';

import { isDevelopment } from '@/constants';
import _ = require('lodash');

@Middleware({ type: 'before' })
class GlobalErrorHandler implements KoaMiddlewareInterface {
  async use(ctx: Context, next: NextKoaMiddleware) {
    try {
      return await next();
    } catch (err) {
      const errBody = {
        errCode: err.code || err.httpCode || err.status,
        errMsg: err.message || err.name
      };

      if (isDevelopment) {
        _.assign(errBody, { stack: err.stack });
      }

      ctx.status = err.httpCode || err.status || 500;
      ctx.body = errBody;

      console.error(err);
    }
  }
}

export default [GlobalErrorHandler];
