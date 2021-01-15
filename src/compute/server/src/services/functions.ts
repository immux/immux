import { FunctionsDoc, FunctionsSchema } from '@/types/models/Functions';
import { AccountSchema } from '@/types/models/Account';
import { HttpError } from 'routing-controllers';
import Functions from '@/models/Functions';
import { Dictionary } from '@/types';
import { Types } from 'mongoose';

import { toObjectId } from '@/utils';
import _ = require('lodash');

export async function getFunctions(
  query: {
    keyword?: string;
    root?: 'true' | 'false';
  },
  account: AccountSchema,
  skip: number,
  limit: number,
  projection?: string | Dictionary<number>
) {
  query = _.defaults(query, {});

  if (_.has(query, ['keyword']) && !_.isString(query.keyword)) {
    throw new HttpError(400, 'invalid keyword');
  }

  const conditions = { creator: account.email }

  const [total, functions] = await Promise.all([
    Functions.countDocuments(conditions),
    Functions.find(conditions)
      .select(projection)
      .sort({ createAt: -1 })
      .skip(skip)
      .limit(limit)
  ]);

  return { total, functions };
}

export async function createFunction(
  creator: AccountSchema,
  data: {
    name: FunctionsDoc['name'];
    projectId: FunctionsDoc['projectId'];
    description?: FunctionsDoc['description'];
    marketStatus?: FunctionsDoc['marketStatus'];
    price?: FunctionsDoc['price'];
  }
) {
  const functions = new Functions(
    _.defaults(
      {
        creator: creator.email,
        createAt: new Date(),
        updater: creator.email,
        updateAt: new Date()
      },
      data
    )
  );

  return functions.save();
}

export function getFunctionById(functionId: Types.ObjectId | string) {
  return Functions.findById(toObjectId(functionId, 'marketFunctions'));
}

export async function updateEditFunction(
  functionId: string | Types.ObjectId,
  updater: AccountSchema,
  data: Partial<
    Pick<FunctionsDoc, 'price' | 'title' | 'description'>
  >
) {
  const editFunction = await getFunctionById(functionId);

  if (!editFunction) {
    throw new HttpError(404, 'editFunction not found');
  }

  editFunction.set({ ...data, updater: updater.email, updateAt: new Date() });
  await editFunction.validateAsync();
  return editFunction.save();
}
