import { FunctionInfo } from '@/types/store/functions';

import { createInstance } from '@/utils/axios';

const instance = createInstance('/api/functions');

/**
 * get PersonFunctions
 * @param creator
 */
export async function fetchPersonFunctions(
  params?: {
    pageNum?: number,
    pageSize?: number 
  }
) {
  return instance.get<
    any,
    {
      functions: FunctionInfo[];
      total: number;
    }
  >(`/`, { params });
}

/**
 * get PublicFunctions
 * @param creator
 */
export async function fetchPublicFunctions(creator: string) {
    return instance.get<
      any,
      {
        functions: FunctionInfo[];
      }
    >(`/${creator}`);
  }
