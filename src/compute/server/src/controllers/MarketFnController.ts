import { getMarketFunctions } from '@/services/market';
import { getFunctionById } from '@/services/functions';
import { PaginationMiddleware } from '@/middlewares/pagination';
import { AccessTokenMiddleware } from '@/middlewares/account';
import { AccountSchema } from '@/types/models/Account';
import { Context } from 'koa';
import * as fs from 'fs';
import * as util from 'util';

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
  Res,
} from 'routing-controllers';

import _ = require('lodash');

const readFileAsync = util.promisify(fs.readFile);

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

  @Get('/download')
  @UseBefore(AccessTokenMiddleware)
  async downloadFn(
    @Ctx() ctx: Context,
  ) {
    const functionId = ctx.query.functionId;

    if (!functionId) {
      throw new HttpError(404, 'function not found');
    }

    const { projectId, name } = await getFunctionById(functionId);

    const filePath = `./uploads/${projectId}/${name}`;
 
    const content = await readFileAsync(filePath);

    const fileType = 'js';

    return {
      name,
      fileType,
      content
    }  
  }
}
