import { FunctionInfo } from '@/types/store/functions';

import { createInstance } from '@/utils/axios';
import { FileInfo } from '@/types/models';

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

export async function updateEditingFunction(
  functionId: string,
  data: {
    title?: string;
    description?: string;
    price?: number;
  }
) {
  return instance.patch<
    any,
    {
      editFunction: FunctionInfo;
    }
  >(`/${functionId}`, data);
}

export async function fetchEditingFunction(functionId: string,) {
  return instance.get<
    any,
    {
      editFunction: FunctionInfo;
    }
  >(`/${functionId}`);
}


/**
 * downloadFunction
 * @param functionId
 */
export async function downloadFunction(functionId: string,) {
  return instanceMarket.get<
    any,
    FileInfo
  >(`/download?functionId=${functionId}`);
}