import { MarketFunctionsDoc, MarketFunctionsSchema } from '@/types/models/MarketFunctions';
import { AccountSchema } from '@/types/models/Account';
import { HttpError } from 'routing-controllers';
import Functions from '@/models/MarketFunctions';
import { Dictionary } from '@/types';
import { Types } from 'mongoose';

import { toObjectId } from '@/utils';
import _ = require('lodash');

export async function getMarketFunctions(
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

  // todo search
  const conditions = {}

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

export async function createMarketFunction(
  creator: AccountSchema,
  data: {
    name: MarketFunctionsDoc['name'];
    projectId: MarketFunctionsDoc['projectId'];
    description?: MarketFunctionsDoc['description'];
    marketStatus?: MarketFunctionsDoc['marketStatus'];
    price?: MarketFunctionsDoc['price'];
  }
) {
  const functions = new Functions(
    _.defaults(
      {
        creator: creator.email,
        createAt: new Date(),
      },
      data
    )
  );

  return functions.save();
}
