import { Account, NameSpace } from '@/types/models';

import { createInstance } from '@/utils/axios';
import { toModel } from '@/utils/models';
import _ from 'lodash';

const instance = createInstance('/api/name-space');

export async function createNameSpace(data: {
  name: string;
  title?: string;
  description?: string;
}) {
  return instance.post<
    any,
    {
      nameSpace: NameSpace;
      accounts: Account[];
    }
  >('', data);
}

export async function updateNameSpace(
  nameSpaceId: string,
  data: {
    title?: string;
    description?: string;
  }
) {
  return instance.patch<
    any,
    {
      nameSpace: NameSpace;
      accounts: Account[];
    }
  >(`/${nameSpaceId}`, data);
}

export async function fetchNameSpaceCollection(params?: {
  pageNum?: number;
  pageSize?: number;
  keyword?: string;
  root?: boolean;
}) {
  const { total, nameSpaces, accounts } = await instance.get<
    any,
    { total: number; nameSpaces: NameSpace[]; accounts: Account[] }
  >('', { params });

  return {
    total,
    accounts: _.map(accounts, (account) => toModel<Account>(account)),
    nameSpaces: _.map(nameSpaces, (nameSpace) => toModel<NameSpace>(nameSpace))
  };
}

export async function fetchNameSpaceProfile(nameSpaceId: string) {
  return instance.get<
    any,
    {
      nameSpace: NameSpace;
      accounts: Account[];
    }
  >(`/${nameSpaceId}`);
}
