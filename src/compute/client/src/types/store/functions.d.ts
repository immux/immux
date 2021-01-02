import { NameSpace, Account, ComputedNameSpace } from '@/types/models';
import { NonUndefined, PromiseType } from 'utility-types';
import { Action, Computed, Thunk } from 'easy-peasy';
import { StoreModel } from '@/types/store';

import { fetchPersonFunctions, fetchPublicFunctions } from '@/services/api/functions';

export interface FunctionInfo {
  id: string;
  projectId: string;
  name: string;
  marketStatus: boolean;
  creator: string;
  price?: number;
  description?: string;
}

export interface FunctionsStoreModel {
  // State
  // --------------------------------------------------------------------------

  total: number;

  personFunctions: FunctionInfo[];

  publicFunctions: FunctionInfo[];

  // Action
  // --------------------------------------------------------------------------

  setTotal: Action<FunctionsStoreModel, number>;

  setPersonFunctions: Action<
    FunctionsStoreModel,
    PromiseType<ReturnType<typeof fetchPersonFunctions>>
  >;

  setPublicFunctions: Action<
    FunctionsStoreModel,
    PromiseType<ReturnType<typeof fetchPublicFunctions>>
  >;

  clear: Action<FunctionsStoreModel>;

  // Thunk
  // --------------------------------------------------------------------------

  fetchPersonFunctions: Thunk<
    FunctionsStoreModel,
    { pageNum?: number,
      pageSize?: number },
    any,
    StoreModel,
    Promise<void>
  >;

  fetchPublicFunctions: Thunk<
    FunctionsStoreModel,
    { creator: string },
    any,
    StoreModel,
    Promise<void>
  >;
}
