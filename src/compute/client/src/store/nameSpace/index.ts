import { NameSpaceStoreModel } from '@/types/store/nameSpace';
import { Account } from '@/types/models';

import { toComputedNameSpace } from '@/services/nameSpace';
import { action, computed, thunk } from 'easy-peasy';
import { collection } from './collection';
import _ from 'lodash';

import {
  fetchNameSpaceProfile,
  updateNameSpace
} from '@/services/api/nameSpace';

export const nameSpace: NameSpaceStoreModel = {
  collection,

  // State
  // --------------------------------------------------------------------------

  nameSpace: undefined,
  accounts: new Map<string, Account>(),

  // Computed
  // --------------------------------------------------------------------------

  entry: computed((state) => {
    // prettier-ignore
    return (
      state.nameSpace ?
        toComputedNameSpace(state.nameSpace, state.accounts as any) :
        undefined
    );
  }),

  // Action
  // --------------------------------------------------------------------------

  setNameSpace: action((state, { nameSpace, accounts }) => {
    state.nameSpace = nameSpace;

    _.forEach(accounts, (account) => {
      state.accounts.set(account.email, account);
    });
  }),

  clearNameSpace: action((state) => {
    state.nameSpace = undefined;
    state.accounts = new Map();
  }),

  // Thunk
  // --------------------------------------------------------------------------

  fetchNameSpace: thunk(async (actions, { nameSpaceId }) => {
    actions.setNameSpace(await fetchNameSpaceProfile(nameSpaceId));
  }),

  // DEPRECATED
  // --------------------------------------------------------------------------

  editingNameSpace: null,

  setEditingNameSpace: action((state, nameSpace) => {
    state.editingNameSpace = nameSpace || null;
  }),

  fetchEditingNameSpace: thunk(async (actions, { nameSpaceId }) => {
    const nameSpaceProfile = await fetchNameSpaceProfile(nameSpaceId);
    const nameSpace = nameSpaceProfile.nameSpace;

    actions.setEditingNameSpace(nameSpace);

    return nameSpace;
  }),

  updateEditingNameSpace: thunk(async (actions, { nameSpaceId, data }) => {
    const { nameSpace } = await updateNameSpace(nameSpaceId, data);

    actions.setEditingNameSpace(nameSpace);

    return nameSpace;
  })
};
