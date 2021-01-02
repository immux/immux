import { getFunctions } from '@/services/functions';
import { PaginationMiddleware } from '@/middlewares/pagination';
import { AccessTokenMiddleware } from '@/middlewares/account';
import { AccountSchema } from '@/types/models/Account';
import { Context } from 'koa';

import {
  JsonController,
  UseBefore,
  HttpError,
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
}
