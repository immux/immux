import { StoreModel } from '@/types/store';

import { createStore } from 'easy-peasy';
import { nameSpace } from './nameSpace';
import { account } from './account';
import { project } from './project';
import { functions } from './functions';

const storeModel: StoreModel = {
  nameSpace,
  account,
  project,
  functions
};

export const store = createStore(storeModel);
