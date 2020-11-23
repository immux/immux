import { Account } from '@/types/models';

import request = require('request-promise-native');
import { apiOrigin } from '@/constants';
import { getConfig } from '@/utils';

import _ from 'lodash';
import FormData = require('form-data');
import * as fs from 'fs';
import * as path from 'path';
import { any } from 'bluebird';

const fetch = require('node-fetch');

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

export function uploadFile() {
  const pathName = `${process.cwd()}/../app/fns`;
  const form = new FormData();

  const config = getConfig();
  form.append('name', config.projectName);

  fs.readdir(pathName, (err, files) => {
    (function iterator(i) {
      if (i == files.length) {
        const url = `${apiOrigin}/cli/upload`;
        return fetch(url, {
          method: 'POST',
          //@ts-ignore
          body: form
        });
      }

      fs.stat(path.join(pathName, files[i]), (err, data) => {
        if (data.isFile()) {
          form.append('files', fs.createReadStream(`${pathName}/${files[i]}`));
        }
        iterator(i + 1);
      });
    })(0);
  });
}
