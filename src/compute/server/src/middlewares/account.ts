import { HttpError, KoaMiddlewareInterface } from 'routing-controllers';
import { NextKoaMiddleware } from '@/types';
import Account from '@/models/Account';
import { Context } from 'koa';
import config from '@/config';

import { getBearerToken, jsonTokenDecode } from '@/utils';
import _ = require('lodash');

import {
  getAccountByEmail,
} from '@/services/account';

export class AccessTokenMiddleware implements KoaMiddlewareInterface {
  async use(ctx: Context, next: NextKoaMiddleware) {
    if (!_.has(ctx.headers, 'authorization')) {
      ctx.throw(401, 'authorization not found');
    }

    const accessToken = getBearerToken(ctx.headers.authorization);

    if (!accessToken) {
      ctx.throw('401', 'invalid authorization');
    }

    const decodeToken = await jsonTokenDecode(accessToken, config.secret);

    if (!decodeToken) {
      ctx.throw('401', 'invalid authorization');
    }
    
    const { email, expiresIn } = JSON.parse(decodeToken);

    const account = await Account.findOne({ email });

    if (!account) {
      ctx.throw(404, 'account not found');
    }

    _.assign(ctx.state, { account, accessToken, expiresIn });

    await next();
  }
}

export class AccessTicketMiddleware implements KoaMiddlewareInterface {
  async use(ctx: Context, next: NextKoaMiddleware) {
    if (!_.has(ctx.headers, 'authorization')) {
      ctx.throw(401, 'authorization not found');
    }

    const accessTicket = getBearerToken(ctx.headers.authorization);

    if (!accessTicket) {
      ctx.throw(401, 'access ticket not exists');
    }

    const decodeToken = await jsonTokenDecode(accessTicket, config.secret);

    if (!decodeToken) {
      ctx.throw('401', 'invalid authorization');
    }

    const { email, expiresIn } = JSON.parse(decodeToken);
    const account = await getAccountByEmail(email);

    if (!account) {
      ctx.throw(404, 'account not found');
    }

    _.assign(ctx.state, { account, accessTicket, expiresIn });

    await next();
  }
}
