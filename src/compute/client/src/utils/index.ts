import moment, { Moment } from 'moment';
import { ReactNode } from 'react';

import { useLocation } from 'react-router-dom';
import { createBrowserHistory } from 'history';
import { message } from 'antd';
import _ from 'lodash';
import qs from 'qs';
import { FileInfo } from '@/types/models';

/**
 * get query hook
 */
export function useQuery<T = Partial<{ [key: string]: any }>>(): T {
  return _.reduce(
    _.split(useLocation().search, '?'),
    (acc, subSearch) => _.assign(acc, qs.parse(subSearch)),
    {} as T
  );
}

/**
 * get queryParam hook
 * @param key
 * @param defaultValue
 */
export function useQueryParam<T = any>(
  key: string | string[],
  defaultValue?: T
): T {
  return _.get<T>(useQuery(), key, defaultValue as T);
}

export function sleep(sleepTime = _.random(1024, 2048)) {
  return new Promise((resolve) => {
    setTimeout(resolve, sleepTime);
  });
}

/**
 * BrowserHistory
 */
export const history = createBrowserHistory();

/**
 * convert `moment` object
 * @param value
 */
export function toMoment(
  value?: string | number | Date | Moment | null
): Moment | null {
  if (!value) {
    return null;
  }

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

/**
 * catch error
 * @param err
 * @param prefix
 * @param msg
 */
export function catchError(err: any, prefix?: ReactNode, msg?: ReactNode) {
  // antd `form.validateFields()` error
  if (_.has(err, ['errorFields']) && _.isArray(err.errorFields)) {
    msg = _.get(err, ['errorFields', 0, 'errors', 0]);
  } else if (_.isNil(msg)) {
    msg = _.get(err, ['response', 'data', 'errMsg'], err.message);
  }

  message.error(prefix ? _.flatten([prefix, ': ', msg]) : msg);

  // eslint-disable-next-line no-console
  console.error(err);
}

export function trimStringValue(value: any) {
  return _.isString(value) ? _.trim(value) : value;
}

export function saveFile(file: FileInfo) {
  let ab = Buffer.from(file.content);
  const blob = new Blob([ab], {
    type: file.fileType
  });
  const filename = file.name;
  const link = document.createElement('a');
  const body = document.querySelector('body');

  if (body) {
    link.href = window.URL.createObjectURL(blob);
    link.download = filename;

    // fix Firefox
    link.style.display = 'none';
    body.appendChild(link);

    link.click();
    body.removeChild(link);

    window.URL.revokeObjectURL(link.href);
  }
}
