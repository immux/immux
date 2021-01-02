import { FunctionInfo } from '@/types/store/functions';

import { createInstance } from '@/utils/axios';

const instance = createInstance('/api/functions');
const instanceMarket = createInstance('/api/marketFn');

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
export async function fetchPublicFunctions(
  params?: {
    pageNum?: number,
    pageSize?: number 
  }
) {
  return instanceMarket.get<
    any,
    {
      functions: FunctionInfo[];
      total: number;
    }
  >(`/`, { params });
}

export async function addFunctionMarket(functionId: string) {
  return instance.post<any, number>(`/addMarket`, {
    functionId,
  });
}
