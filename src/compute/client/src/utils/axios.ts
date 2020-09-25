import { history } from '@/utils';
import moment from 'moment';
import axios from 'axios';
import _ from 'lodash';
import qs from 'qs';

/**
 * throwError
 * @param error
 */
function throwError<T = any>(error: T) {
  return Promise.reject<T>(error);
}

/**
 * createInstance
 * @param baseURL Prefix
 */
export function createInstance(baseURL: string) {
  const instance = axios.create({
    baseURL,
    withCredentials: true,

    paramsSerializer(params) {
      return qs.stringify(params, { arrayFormat: 'repeat' });
    }
  });

  instance.interceptors.request.use((config) => {
    if (`${config.baseURL}${config.url}` === '/api/account/login') {
      return config;
    }

    const { store } = require('@/store');
    const { accessToken, expiresIn } = store.getState().account;

    if (!accessToken || !expiresIn || moment(expiresIn).isBefore(Date.now())) {
      history.push('/account/login');
      throw new Error('Login has expired, please log in again');
    }

    _.set(config, ['headers', 'Authorization'], `Bearer ${accessToken}`);

    return config;
  }, throwError);

  instance.interceptors.response.use(
    (res) => res.data,
    (err) => {
      const status: number = _.get(err, ['response', 'status']);
      const errCode: number = _.get(err, ['response', 'data', 'errCode']);
      const errMsg: string = _.get(err, ['response', 'data', 'errMsg']);

      if (
        status === 401 &&
        errCode === 401 &&
        errMsg === 'access token not exists'
      ) {
        history.push('/account/login');
        throw new Error('Login has expired, please log in again');
      }

      return throwError(err);
    }
  );

  return instance;
}
