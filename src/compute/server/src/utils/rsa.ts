import NodeRSA = require('node-rsa');

import { oreCharacters, PUBLIC_PEM_REGEX } from '@/constants';
import { Types } from 'mongoose';
import _ = require('lodash');

/**
 * @desc (https://www.shangyang.me/2017/05/24/encrypt-rsa-keyformat/)
 */
export function generateRsaPems() {
  const key = new NodeRSA({ b: 512 });

  key.setOptions({ encryptionScheme: 'pkcs1' });

  const privatePem = key.exportKey('pkcs1-private-pem');
  const publicPem = key.exportKey('pkcs1-public-pem');

  return { publicPem, privatePem };
}

export function encryptRawText(privatePem: string | Buffer, rawText: string) {
  return new NodeRSA(privatePem).encryptPrivate(rawText, 'base64');
}

export function decryptText(publicPem: string | Buffer, encryptedText: string) {
  return new NodeRSA(publicPem).decryptPublic(encryptedText, 'utf8');
}

export function isValidPublicPem(publicPem: string) {
  return PUBLIC_PEM_REGEX.test(publicPem);
}

export function genRawText() {
  return `${new Types.ObjectId().toHexString()}::${_.sample(oreCharacters)}`;
}

