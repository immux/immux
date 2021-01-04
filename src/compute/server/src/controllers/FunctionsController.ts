import { getFunctions, getFunctionById, updateEditFunction } from '@/services/functions';
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
export default class FunctionsController {
  @Get('')
  @UseBefore(AccessTokenMiddleware, PaginationMiddleware)
  async getMyFunctions(
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

  @Get('/:functionId')
  @UseBefore(AccessTokenMiddleware)
  async getEditFunction(@Param('functionId') functionId: string) {
    const editFunction = await getFunctionById(functionId);

    if (!editFunction) {
      throw new HttpError(404, 'editFunction not found');
    }

    return { editFunction };
  }

  @Patch('/:id')
  @UseBefore(AccessTokenMiddleware)
  async updateEditFunction(
    @State('account') account: AccountSchema,
    @Param('id') functionId: string,
    @Body() data: Parameters<typeof updateEditFunction>[2]
  ) {
    const editFunction = await updateEditFunction(functionId, account, data);
    return { editFunction };
  }
}
