import { NameSpace, Account, ComputedNameSpace } from '@/types/models';
import { NonUndefined, PromiseType } from 'utility-types';
import { Action, Computed, Thunk } from 'easy-peasy';
import { StoreModel } from '@/types/store';

import {
  fetchNameSpaceCollection,
  fetchNameSpaceProfile
} from '@/services/api/nameSpace';

export interface NameSpaceCollectionStoreModel {
  // State
  // --------------------------------------------------------------------------
  total: number;

  // nameSpaces list
  nameSpaces: NameSpace[];

  accounts: Map<string, Account>;

  // Computed
  // --------------------------------------------------------------------------

  entries: Computed<
    NameSpaceCollectionStoreModel,
    ComputedNameSpace[],
    StoreModel
  >;

  // Action
  // --------------------------------------------------------------------------

  setTotal: Action<NameSpaceCollectionStoreModel, number>;

  addNameSpaces: Action<
    NameSpaceCollectionStoreModel,
    [NameSpace[], Account[]]
  >;

  addRootNameSpace: Action<
    NameSpaceCollectionStoreModel,
    [NameSpace, Account[]]
  >;

  clear: Action<NameSpaceCollectionStoreModel>;

  // Thunk
  // --------------------------------------------------------------------------

  fetchNameSpaces: Thunk<
    NameSpaceCollectionStoreModel,
    NonUndefined<Parameters<typeof fetchNameSpaceCollection>[0]>,
    any,
    StoreModel,
    Promise<void>
  >;
}

export interface NameSpaceStoreModel {
  // Modules
  // --------------------------------------------------------------------------

  collection: NameSpaceCollectionStoreModel;

  // State
  // --------------------------------------------------------------------------

  nameSpace?: NameSpace;

  accounts: Map<string, Account>;

  // Computed
  // --------------------------------------------------------------------------

  entry: Computed<NameSpaceStoreModel, ComputedNameSpace | undefined>;

  // Action
  // --------------------------------------------------------------------------

  setNameSpace: Action<
    NameSpaceStoreModel,
    PromiseType<ReturnType<typeof fetchNameSpaceProfile>>
  >;

  clearNameSpace: Action<NameSpaceStoreModel, void>;

  // Thunk
  // --------------------------------------------------------------------------

  fetchNameSpace: Thunk<
    NameSpaceStoreModel,
    { nameSpaceId: string },
    any,
    StoreModel,
    Promise<void>
  >;

  // DEPRECATED
  // --------------------------------------------------------------------------

  editingNameSpace: NameSpace | null;

  setEditingNameSpace: Action<NameSpaceStoreModel, NameSpace | null>;

  fetchEditingNameSpace: Thunk<
    NameSpaceStoreModel,
    { nameSpaceId: string },
    any,
    StoreModel
  >;

  updateEditingNameSpace: Thunk<
    NameSpaceStoreModel,
    { nameSpaceId: string; data: { title?: string; description?: string } },
    any,
    StoreModel
  >;
}
