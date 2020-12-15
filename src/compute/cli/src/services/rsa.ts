import { genRsaSignature, verifyRsaSignature } from '@/services/api';
import { decryptSignature } from '@/utils/rsa';
import * as util from 'util';
import chalk from 'chalk';
import * as fs from 'fs';

import {
  PUBLIC_PEM_REGEXP,
  publicPemPath,
  publicPemFilename
} from '@/constants';

const readFileAsync = util.promisify(fs.readFile);

function isValidPublicPem(content: string) {
  return PUBLIC_PEM_REGEXP.test(content);
}

export async function getPublicPem() {
  if (!fs.existsSync(publicPemPath)) {
    throw new Error(`PublicPem ${chalk.gray(`~/${publicPemFilename}`)} does not exist`);
  }

  const content = await readFileAsync(publicPemPath, { encoding: 'utf8' });

  if (!isValidPublicPem(content)) {
    throw new Error(
      `PublicPem ${chalk.gray(`~/${publicPemFilename}`)} wrong format`
    );
  }

  const [, email, publicPem] = content.match(
    PUBLIC_PEM_REGEXP
  ) as RegExpMatchArray;

  return { email, publicPem };
}

export async function getAccessTicket(email: string, publicPem: string) {
  const { signature } = await genRsaSignature(email, publicPem);

  const rawText = decryptSignature(publicPem, signature);

  const { account, ticket } = await verifyRsaSignature(
    email,
    publicPem,
    signature,
    rawText
  );

  return { ticket, account };
}
