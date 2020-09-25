import { DocumentToObjectOptions } from 'mongoose';

import _ = require('lodash');

const EXCLUDE_KEYS = ['__v'];

function transformRet(ret: any): any {
  return _.reduce(
    ret,
    (ret, value, key) => {
      if (_.includes(EXCLUDE_KEYS, key)) {
        return ret;
      }

      if (key === '_id') {
        key = 'id';
      } else if (_.isDate(value)) {
        value = +value;
      } else if (_.isPlainObject(value)) {
        value = transformRet(value);
      }

      ret[key] = value;

      return ret;
    },
    {} as { [key: string]: any }
  );
}

export function genJsonHandler(
  ...handlers: ((doc: any, ret: any, opts: any) => any)[]
) {
  return {
    virtuals: true,

    transform(doc, ret, opts) {
      return _.reduce(
        handlers,
        (ret, handler) => handler(doc, ret, opts),
        transformRet(ret)
      );
    }
  } as DocumentToObjectOptions;
}
