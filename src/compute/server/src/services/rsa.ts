import { HttpError } from 'routing-controllers';

import { existsAccountPems, getAccountPems } from '@/services/account';
import genCacheKey, { CacheKeys } from '@/cache';
import { isValidPublicPem } from '@/utils/rsa';
import { isValidEmail } from '@/utils';
import _ = require('lodash');

export async function getPems(email: string, publicPem: string) {
  if (!isValidEmail(email)) {
    throw new HttpError(400, 'invalid email');
  }

  if (!isValidPublicPem(publicPem)) {
    throw new HttpError(400, 'invalid publicPem');
  }

  // if (!(await existsAccountPems(email))) {
  //   throw new HttpError(404, 'pems not found');
  // }

  const pems = await getAccountPems(email);

  // if (pems.email !== email) {
  //   throw new HttpError(403, 'email not match');
  // }

  // if (pems.publicPem !== publicPem) {
  //   throw new HttpError(403, 'publicPem not match');
  // }

  return pems;
}

export function saveRsaRawText(
  email: string,
  publicPem: string,
  signature: string,
  rawText: string
) {
  return genCacheKey(CacheKeys.RsaRawText)(email, publicPem, signature);
  // return client.set(
  //   genCacheKey(CacheKeys.RsaRawText)(email, publicPem, signature),
  //   rawText,
  //   'EX',
  //   16
  // );
}

export async function getRsaRawText(
  email: string,
  publicPem: string,
  signature: string
): Promise<string> {
  return genCacheKey(CacheKeys.RsaRawText)(email, publicPem, signature);
  // return _.get(
  //   await client
  //     .multi()
  //     .get(genCacheKey(CacheKeys.RsaRawText)(email, publicPem, signature))
  //     .del(genCacheKey(CacheKeys.RsaRawText)(email, publicPem, signature))
  //     .exec(),
  //   [0, 1]
  // );
}
