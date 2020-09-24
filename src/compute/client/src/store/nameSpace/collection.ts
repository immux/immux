import { NameSpaceCollectionStoreModel } from '@/types/store/nameSpace';

import { fetchNameSpaceCollection } from '@/services/api/nameSpace';
import { toComputedNameSpace } from '@/services/nameSpace';
import { action, computed, thunk } from 'easy-peasy';
import _ from 'lodash';

export const collection: NameSpaceCollectionStoreModel = {
  // State
  // --------------------------------------------------------------------------

  total: 0,
  nameSpaces: [],
  accounts: new Map(),

  // Computed
  // --------------------------------------------------------------------------

  entries: computed((state) => {
    // prettier-ignore
    return _.map(
      state.nameSpaces,
      nameSpace => toComputedNameSpace(nameSpace, state.accounts as any)
    );
  }),

  // Action
  // --------------------------------------------------------------------------

  setTotal: action((state, total = 0) => {
    state.total = total;
  }),

  addNameSpaces: action((state, [nameSpaces, accounts]) => {
    _.forEach(accounts, (account) => {
      state.accounts.set(account.email, account);
    });

    state.nameSpaces = _.concat(
      state.nameSpaces,
      _.reject(nameSpaces, ['root', true])
    );
  }),

  addRootNameSpace: action((state, [nameSpace, accounts]) => {
    if (
      !nameSpace ||
      !nameSpace.root ||
      (state.nameSpaces[0] && state.nameSpaces[0].root)
    ) {
      return;
    }

    _.forEach(accounts, (account) => {
      state.accounts.set(account.email, account);
    });

    state.nameSpaces = _.concat([nameSpace], state.nameSpaces);
  }),

  clear: action((state) => {
    state.total = 0;
    state.nameSpaces = [];
    state.accounts.clear();
  }),

  // Thunk
  // --------------------------------------------------------------------------

  fetchNameSpaces: thunk(async (actions, params, { getState }) => {
    const { nameSpaces: stateNameSpaces } = getState();

    if (!stateNameSpaces[0] || !stateNameSpaces[0].root) {
      const {
        nameSpaces: [nameSpace],
        accounts
      } = await fetchNameSpaceCollection(_.defaults({ root: true }, params));

      actions.addRootNameSpace([nameSpace, accounts]);
    }

    const { total, nameSpaces, accounts } = await fetchNameSpaceCollection(
      params
    );

    actions.setTotal(total);
    actions.addNameSpaces([nameSpaces, accounts]);
  })
};
