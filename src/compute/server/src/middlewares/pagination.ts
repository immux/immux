import { KoaMiddlewareInterface } from 'routing-controllers';
import { NextKoaMiddleware } from '@/types';
import { Context } from 'koa';

import _ = require('lodash');

export class PaginationMiddleware implements KoaMiddlewareInterface {
  async use(ctx: Context, next: NextKoaMiddleware) {
    const pageNum = _.toNumber(
      _.has(ctx.query, ['pageNum']) ? ctx.query.pageNum : 1
    );

    const pageSize = _.toNumber(
      _.has(ctx.query, ['pageSize']) ? ctx.query.pageSize : 10
    );

    if (!_.isInteger(pageNum)) {
      ctx.throw(400, 'invalid pageNum');
    }

    if (!_.isInteger(pageSize)) {
      ctx.throw(400, 'invalid pageSize');
    }

    const limit = pageSize;
    const skip = (pageNum - 1) * pageSize;

    _.assign(ctx.state, { pageNum, pageSize, limit, skip });

    await next();
  }
}
