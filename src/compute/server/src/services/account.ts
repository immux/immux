import { PromiseType } from 'utility-types';
import Account from '@/models/Account';
import { AccountPems } from '@/types';
import config from '@/config';

import { getGitAccountProfile } from '@/services/git';
import { getEmailsSet, getTextHash, jsonTokenEncode } from '@/utils';
import { generateRsaPems } from '@/utils/rsa';
import genCacheKey, { CacheKeys } from '@/cache';
import moment = require('moment');
import _ = require('lodash');

export async function removeAccessToken(accessToken: string) {
  const cacheKey = genCacheKey(CacheKeys.AccessToken);
  // todo failure token
}

export async function genAccessToken(email: string) {
  const expiresIn = +moment().add(24, 'hours');

  const accessToken = await jsonTokenEncode({
    email,
    expiresIn,
  }, config.secret);

  return { email, accessToken, expiresIn };
}

export function findOneAndUpdateAccount(
  account: PromiseType<ReturnType<typeof getGitAccountProfile>>
) {
  return Account.findOneAndUpdate(
    { email: account.email },
    {
      $setOnInsert: { 
        email: account.email, 
        createAt: new Date(), 
        avatar: account.avatarUrl ,
        name: account.login,
      },
    },
    { new: true, upsert: true }
  );
}

export function getAccountByEmail(email: string) {
  return Account.findOne({ email });
}

export function getAccountsByEmails(emails: string[] | Set<string>) {
  return Account.find({ email: { $in: Array.from(emails) } });
}

export function getAccounts(...docs: any[]) {
  return getAccountsByEmails(getEmailsSet(...docs));
}

export async function genAccessTicket(email: string) {
  const expiresIn = +moment().add(15, 'minutes');

  const ticket = await jsonTokenEncode({
    email,
    expiresIn,
  }, config.secret);

  return { email, ticket, expiresIn };
}

export function genAccountPems(email: string): AccountPems {
  const { privatePem, publicPem } = generateRsaPems();

  return {
    hash: getTextHash(privatePem),
    email,
    privatePem,
    publicPem,
    createAt: Date.now()
  };
}

export async function getAccountPems(email: string) {
  // todo cli database
  // return JSON.parse(
  //   await client.get(genCacheKey(CacheKeys.AccountPems)(email))
  // ) as AccountPems;
}

export async function existsAccountPems(email: string) {
  // return !!(await client.exists(genCacheKey(CacheKeys.AccountPems)(email)));
}

export function toResAccountPems(pems?: AccountPems) {
  return {
    hash: pems.hash,
    email: pems.email,
    publicPem: `-----EMAIL ${pems.email}\n${pems.publicPem}`,
    createAt: pems.createAt
  };
}

export function destroyAccountPems(pems: AccountPems) {
  const toKey = genCacheKey(CacheKeys.AccountPems);

  // return client
  //   .multi()
  //   .del(toKey(pems.hash))
  //   .del(toKey(pems.email))
  //   .del(toKey(pems.privatePem))
  //   .del(toKey(pems.publicPem))
  //   .exec();
}

export function saveAccountPems(pems: AccountPems) {
  const pemsJson = JSON.stringify(pems);
  const toKey = genCacheKey(CacheKeys.AccountPems);
  
  // return client
  //   .multi()
  //   .set(toKey(pems.hash), pemsJson)
  //   .set(toKey(pems.email), pemsJson)
  //   .set(toKey(pems.privatePem), pemsJson)
  //   .set(toKey(pems.publicPem), pemsJson)
  //   .exec();
}

export function toResPublicPem(pems: AccountPems) {
  return `-----EMAIL ${pems.email}\n${pems.publicPem}`;
}


