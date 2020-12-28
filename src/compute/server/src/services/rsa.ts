import { HttpError } from 'routing-controllers';

import { getAccountPems } from '@/services/account';
import genCacheKey, { CacheKeys } from '@/cache';
import { isValidPublicPem } from '@/utils/rsa';
import { isValidEmail } from '@/utils';
import _ = require('lodash');
import Pem from '@/models/Pem';

export async function getPems(email: string, publicPem: string) {
  if (!isValidEmail(email)) {
    throw new HttpError(400, 'invalid email');
  }

  if (!isValidPublicPem(publicPem)) {
    throw new HttpError(400, 'invalid publicPem');
  }

  const pems = await getAccountPems(email);

  if (pems.email !== email) {
    throw new HttpError(403, 'email not match');
  }

  if (pems.publicPem !== publicPem) {
    throw new HttpError(403, 'publicPem not match');
  }

  return pems;
}

export function saveRsaRawText(
  email: string,
  publicPem: string,
  signature: string,
  rawText: string
) {
  return Pem.findOneAndUpdate(
    { email },
    { $set: { rawText } },
    { new: true }
  );
}

export async function getRsaRawText(
  email: string,
  publicPem: string,
  signature: string
) {
  return Pem.findOne({ email });
}
