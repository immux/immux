import { getFunctions, getFunctionById } from '@/services/functions';
import { createMarketFunction } from '@/services/market';
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

import { getEmailsSet } from '@/utils';
import config from '@/config';
import _ = require('lodash');

@JsonController('/functions')
export default class NameSpaceController {
  @Get('')
  @UseBefore(AccessTokenMiddleware, PaginationMiddleware)
  async getNameSpaces(
    @Ctx() ctx: Context,
    @State('skip') skip: number,
    @State('limit') limit: number,
    @State('account') account: AccountSchema,
  ) {
    const { total, functions } = await getFunctions(ctx.query, account, skip, limit);

    return { total, functions };
  }

  /**
   * addMarket
   */
  @Post('/addMarket')
  @UseBefore(AccessTokenMiddleware)
  async addMarket(
    @BodyParam('functionId') functionId: string,
    @State('account') account: AccountSchema,
  ) {
    // find function
    const addFunction = await getFunctionById(functionId);

    // add function to market
    const result = await createMarketFunction(account, addFunction);
    
    return result;
  }
}
