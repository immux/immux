import { createInstance } from '@/utils/axios';

const instance = createInstance('/api/cleansing');

/**
 * clean `db`
 */
export function cleansingFileSpaceOrigin() {
  return instance.post<any, number>('/removeAll');
}
