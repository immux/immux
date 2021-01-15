import { NameSpaceStoreModel } from './nameSpace';
import { AccountStoreModel } from './account';
import { ProjectStoreModel } from './project';
import { FunctionsStoreModel } from './functions';

export interface StoreModel {
  nameSpace: NameSpaceStoreModel;
  account: AccountStoreModel;
  project: ProjectStoreModel;
  functions: FunctionsStoreModel;
}
