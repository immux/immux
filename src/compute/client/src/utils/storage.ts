import _ from 'lodash';

/**
 * localStorage get
 * @param key
 * @param defaultValue
 */
export function getItem<T extends any>(key: string, defaultValue?: T) {
  try {
    const value: T = JSON.parse(localStorage.getItem(key) as string);

    return _.isNil(value) ? (defaultValue as T) : value;
  } catch (err) {
    return defaultValue as T;
  }
}

/**
 * localStorage set
 * @param key
 * @param value
 */
export function setItem<T extends object>(key: string, value: T) {
  localStorage.setItem(key, JSON.stringify(value));
}

/**
 * localStorage remove
 * @param key
 */
export function removeItem(key: string): void {
  localStorage.removeItem(key);
}
