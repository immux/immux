import { Account } from '@/types/models';

import request = require('request-promise-native');
import { apiOrigin } from '@/constants';
import _ from 'lodash';

export function genRsaSignature(email: string, publicPem: string) {
  return request.post({
    url: `${apiOrigin}/cli/rsa/signature`,
    json: true,
    body: { email, publicPem }
  }) as Promise<{ signature: string }>;
}

export function verifyRsaSignature(
  email: string,
  publicPem: string,
  signature: string,
  rawText: string
) {
  return request.post({
    url: `${apiOrigin}/cli/rsa/signature/verify`,
    json: true,
    body: { email, publicPem, signature, rawText }
  }) as Promise<{
    email: string;
    ticket: string;
    expiresIn: number;
    account: Account;
  }>;
}
