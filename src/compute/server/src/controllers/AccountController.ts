import { AccessTokenMiddleware } from '@/middlewares/account';
import { AccountSchema } from '@/types/models/Account';

import {
  JsonController,
  HeaderParam,
  BodyParam,
  HttpError,
  UseBefore,
  State,
  Post,
  Get,
  Delete
} from 'routing-controllers';

import { getGitAccountAccessToken, getGitAccountProfile } from '@/services/git';
import { isDevelopment } from '@/constants';
import { getBearerToken } from '@/utils';
import _ = require('lodash');

import {
  findOneAndUpdateAccount,
  destroyAccountPems,
  removeAccessToken,
  existsAccountPems,
  toResAccountPems,
  saveAccountPems,
  genAccessToken,
  getAccountPems,
  genAccountPems,
  toResPublicPem
} from '@/services/account';

@JsonController('/account')
export default class AccountController {
  @Get('')
  @UseBefore(AccessTokenMiddleware)
  getAccountProfile(@State('account') account: AccountSchema) {
    return account;
  }

  @Post('/login')
  async login(
    @BodyParam('code') code: string,
    @BodyParam('redirectUri') redirectUri: string
  ) {
    if (!_.isString(code)) {
      throw new HttpError(400, 'invalid code');
    }

    if (!_.isString(redirectUri)) {
      throw new HttpError(400, 'invalid redirectUri');
    }

    const {
      accessToken: gitAccessToken,
    } = await getGitAccountAccessToken(code, redirectUri);

    const gitAccount = await getGitAccountProfile(gitAccessToken);

    const account = await findOneAndUpdateAccount(gitAccount);
    const { accessToken, expiresIn } = await genAccessToken(account.email);

    return { account, accessToken, expiresIn };
  }

  @Post('/logout')
  @UseBefore(AccessTokenMiddleware)
  async logout(@HeaderParam('authorization') authorization: string) {
    if (authorization) {
      const accessToken = getBearerToken(authorization);

      if (accessToken) await removeAccessToken(accessToken);
    }

    return 'ok';
  }

  @Get('/pems')
  @UseBefore(AccessTokenMiddleware)
  async getPems(@State('account') account: AccountSchema) {
    // if (!(await existsAccountPems(account.email))) {
    //   return {};
    // }

    const pems = await getAccountPems(account.email);
    // todo cli database
    //@ts-ignorets-ignore
    const publicPem = toResPublicPem(pems);

    return _.defaults(
      { publicPem },
      _.pick(pems, ['hash', 'email', 'createAt'])
    );
  }

  @Post('/pems')
  @UseBefore(AccessTokenMiddleware)
  async genPems(@State('account') account: AccountSchema) {
    // if (await existsAccountPems(account.email)) {
    //   return await getAccountPems(account.email);
    // }

    const pems = genAccountPems(account.email);
    const resPems = toResAccountPems(pems);

    await saveAccountPems(pems);

    // prettier-ignore
    return (
      isDevelopment ?
        { ...resPems, privatePem: pems.privatePem } :
        resPems
    );
  }

  @Delete('/pems')
  @UseBefore(AccessTokenMiddleware)
  async destroyPems(@State('account') account: AccountSchema) {
    // if (!(await existsAccountPems(account.email))) {
    //   throw new HttpError(404, 'account pems not found');
    // }

    // await destroyAccountPems(await getAccountPems(account.email));

    return { message: 'ok' };
  }
}
