import { Aggregate, Error, Types } from 'mongoose';
import { HttpError } from 'routing-controllers';
import { Moment } from 'moment';

import createDebugger, { Debugger } from 'debug';
import { safeLoad } from 'js-yaml';
import * as fs from 'fs';
import * as path from 'path';

import { EMAIL_REGEX } from '@/constants';
import moment = require('moment');
import crypto = require('crypto');
import _ = require('lodash');
import { jsonTokenInfo } from '@/types';

export const debug = {
  cleansing: createDebugger('cleansing') as Debugger,
  mongo: createDebugger('mongodb') as Debugger,
  app: createDebugger('app') as Debugger
};

export function toMoment(
  value: string | number | Date | Moment
): Moment | null {
  if (moment.isMoment(value)) {
    return value.clone();
  }

  let instance;

  if (
    _.isNumber(value) &&
    _.size(`${value}`) >= 10 &&
    (instance = moment(+_.padEnd(`${value}`, 13, '0'))).isValid()
  ) {
    return instance;
  }

  if (_.isDate(value) && (instance = moment(value)).isValid()) {
    return instance;
  }

  if ((instance = moment(value)).isValid()) {
    return instance;
  }

  return null;
}

export function requireYaml<T = any>(filePath: string) {
  // @ts-ignore
  return safeLoad(fs.readFileSync(filePath, { encoding: 'utf8' })) as T;
}

export function randomBase64() {
  const id = new Types.ObjectId().toString();
  const md5 = crypto.createHash('md5').update(id).digest('hex'); // prettier-ignore

  return Buffer.from(md5).toString('base64');
}

export function getBearerToken(authorization: string) {
  const parts = authorization.split(' ');
  return parts.length === 2 && parts[0] === 'Bearer' ? parts[1] : null;
}

function addEmailsToSet(emails: Set<string>, doc: any) {
  for (const emailProp of ['creator', 'updater', 'archivist']) {
    const email = _.get(doc, [emailProp]);

    if (email) {
      emails.add(email);
    }
  }
}

export function getEmailsSet(...docs: any[]) {
  const emails = new Set<string>();

  _.forEach(docs, doc => {
    if (!_.isArray(doc)) {
      doc = [doc];
    }

    _.forEach(doc, addEmailsToSet.bind(null, emails));
  });

  return emails;
}

export function getTextHash(text: string) {
  return crypto
    .createHash('md5')
    .update(text)
    .digest('hex');
}

export function toObjectId(id: any, errMsg = 'invalid id') {
  if (!Types.ObjectId.isValid(id)) {
    throw new HttpError(
      400,
      _.startsWith(errMsg, 'invalid') ? errMsg : `invalid ${errMsg} id`
    );
  }

  return new Types.ObjectId(id);
}

export function isValidEmail(email: string) {
  return EMAIL_REGEX.test(email);
}

export async function getAggregateTotal(
  aggregate: (...args: any[]) => Aggregate<any[]>,
  ...args: any[]
) {
  const aggregated = await aggregate(...args)
    .group({ _id: null, total: { $sum: 1 } })
    .project({ _id: 0 });

  // prettier-ignore
  return (
      _.isEmpty(aggregated)
        ? 0
        : _.get(aggregated, [0, 'total'])
    ) as number;
}

/**
 * @example
 * schema.methods.validateAsync = validateMongooseDocument;
 */
export function validateMongooseDocument(): Promise<void> {
  return this.validate().catch((validationError: Error.ValidationError) => {
    const error = _.first(_.values(validationError.errors));

    if (error.reason) {
      throw new HttpError(
        _.get(error.reason, ['httpCode'], 400),
        error.reason.message
      );
    }

    switch (error.kind) {
      case 'required': {
        throw new HttpError(400, `required ${error.path}`);
      }

      case 'enum':
      default: {
        throw new HttpError(400, `invalid ${error.path}`);
      }
    }
  });
}

export async function base64urlEscape(str: string) {
  return str.replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');
}

export async function base64urlUnescape(str: string) {
  str += new Array(5 - str.length % 4).join('=');
  return str.replace(/\-/g, '+').replace(/_/g, '/');
}

export async function toBase64(content: object) {
  return base64urlEscape(Buffer.from(JSON.stringify(content)).toString('base64'));
}

export async function jsonTokenEncode(info: jsonTokenInfo, secret: string) {
  let header = await toBase64({ typ: 'JWT', alg: 'HS256' });
  let content = await toBase64(info);
  let sign = await jsonTokenSign([header, content].join('.'), secret);
  
  return [header, content, sign].join('.')
}

export async function jsonTokenDecode(token: string, secret: string) {
  let [header, content, sign] = token.split('.');
  let newSign = await jsonTokenSign([header, content].join('.'), secret);
  
  if (sign === newSign) {
    const result = await base64urlUnescape(content)
    return Buffer.from(result, 'base64').toString();
  }
}

export async function jsonTokenSign(content: string, secret: string) {
  let result = crypto.createHmac('sha256',secret).update(content).digest('base64');
  return base64urlEscape(result);
}


export function getStat(path: string): Promise<false | fs.Stats> {
  return new Promise((resolve, reject) => {
      return fs.stat(path, (err, stats) => {
          if(err){
              resolve(false);
          }else{
              resolve(stats);
          }
      })
  })
}  

export function mkdir(dir: string){
  return new Promise((resolve, reject) => {
      fs.mkdir(dir, err => {
          if(err){
              resolve(false);
          }else{
              resolve(true);
          }
      })
  })
}

export async function dirExists(dir: string){
  let isExists = await getStat(dir);

  if(isExists && isExists.isDirectory()) {
      return true;
  } else if(isExists){
      return false;
  }

  let tempDir = path.parse(dir).dir;

  let status = await dirExists(tempDir);
  let mkdirStatus;

  if(status){
      mkdirStatus = await mkdir(dir);
  }

  return mkdirStatus;
}
