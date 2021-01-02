import { getMarketFunctions } from '@/services/market';
import { PaginationMiddleware } from '@/middlewares/pagination';
import { AccessTokenMiddleware } from '@/middlewares/account';
import { AccountSchema } from '@/types/models/Account';
import { Context } from 'koa';

import {
  JsonController,
  UseBefore,
  HttpError,
  BodyParam,
  State,
  Param,
  Patch,
  Post,
  Body,
  Get,
  Ctx,
} from 'routing-controllers';

import _ = require('lodash');

@JsonController('/marketFn')
export default class MarketFnController {
  @Get('')
  @UseBefore(AccessTokenMiddleware, PaginationMiddleware)
  async getMarketFns(
    @Ctx() ctx: Context,
    @State('skip') skip: number,
    @State('limit') limit: number,
    @State('account') account: AccountSchema,
  ) {
    const { total, functions } = await getMarketFunctions(ctx.query, account, skip, limit);

    return { total, functions };
  }
}
