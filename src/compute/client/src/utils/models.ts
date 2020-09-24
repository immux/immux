import { toMoment } from '@/utils/index';
import _ from 'lodash';

export function toModel<T>(doc: T) {
  const model: T = { ...doc };

  _.forEach(['createAt', 'updateAt'], (prop) => {
    if (_.has(model, [prop])) {
      // @ts-ignore
      _.set(model, [prop], toMoment(model[prop]));
    }
  });

  return model;
}
