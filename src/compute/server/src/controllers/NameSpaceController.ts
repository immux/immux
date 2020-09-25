import { getAccounts, getAccountsByEmails } from '@/services/account';
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

import {
  getNameSpaceById,
  createNameSpace,
  updateNameSpace,
  getNameSpaces
} from '@/services/nameSpace';

@JsonController('/name-space')
export default class NameSpaceController {
  @Get('')
  @UseBefore(AccessTokenMiddleware, PaginationMiddleware)
  async getNameSpaces(
    @Ctx() ctx: Context,
    @State('skip') skip: number,
    @State('limit') limit: number
  ) {
    const { total, nameSpaces } = await getNameSpaces(ctx.query, skip, limit);
    const accounts = await getAccountsByEmails(getEmailsSet(nameSpaces));

    return { total, nameSpaces, accounts };
  }

  @Get('/:nameSpaceId')
  @UseBefore(AccessTokenMiddleware)
  async getNameSpace(@Param('nameSpaceId') nameSpaceId: string) {
    const nameSpace = await getNameSpaceById(nameSpaceId);

    if (!nameSpace) {
      throw new HttpError(404, 'nameSpace not found');
    }

    const accounts = await getAccountsByEmails(getEmailsSet(nameSpace));

    return { nameSpace, accounts };
  }

  @Post('')
  @UseBefore(AccessTokenMiddleware)
  async createNameSpace(
    @State('account') account: AccountSchema,
    @Body()
    data: Omit<Parameters<typeof createNameSpace>[1], ''>
  ) {
    const nameSpace = await createNameSpace(
      account,
      _.defaults({}, data)
    );

    return { nameSpace, accounts: await getAccounts(nameSpace) };
  }

  @Patch('/:id')
  @UseBefore(AccessTokenMiddleware)
  async updateNameSpace(
    @State('account') account: AccountSchema,
    @Param('id') nameSpaceId: string,
    @Body() data: Parameters<typeof updateNameSpace>[2]
  ) {
    const nameSpace = await updateNameSpace(nameSpaceId, account, data);
    return { nameSpace, accounts: await getAccounts(nameSpace) };
  }
}
