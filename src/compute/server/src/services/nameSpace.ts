import { NameSpaceDoc, NameSpaceSchema } from '@/types/models/NameSpace';
import { AccountSchema } from '@/types/models/Account';
import { HttpError } from 'routing-controllers';
import NameSpace from '@/models/NameSpace';
import { Dictionary } from '@/types';
import { Types } from 'mongoose';

import { ROOT_NAME } from '@/constants';
import { toObjectId } from '@/utils';
import _ = require('lodash');

export async function getNameSpaces(
  query: {
    keyword?: string;
    root?: 'true' | 'false';
  },
  skip: number,
  limit: number,
  projection?: string | Dictionary<number>
) {
  query = _.defaults(query, {});

  if (_.has(query, ['keyword']) && !_.isString(query.keyword)) {
    throw new HttpError(400, 'invalid keyword');
  }

  const conditions = {}

  if (_.has(query, ['root'])) {
    _.assign(conditions, {
      name: { [query.root === 'true' ? '$eq' : '$ne']: ROOT_NAME }
    });
  }

  const [total, nameSpaces] = await Promise.all([
    NameSpace.countDocuments(conditions),
    NameSpace.find(conditions)
      .select(projection)
      .sort({ createAt: -1 })
      .skip(skip)
      .limit(limit)
  ]);

  return { total, nameSpaces };
}

export async function existsNameSpace(
  criteria: Parameters<typeof NameSpace.countDocuments>[0]
) {
  return NameSpace.countDocuments(criteria).then(count => !!count);
}

export async function createNameSpace(
  creator: AccountSchema,
  data: {
    name: NameSpaceDoc['name'];
    title?: NameSpaceDoc['title'];
    description?: NameSpaceDoc['description'];
  }
) {
  const nameSpace = new NameSpace(
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

  await nameSpace.validateAsync();

  if (await existsNameSpace(_.pick(nameSpace, ['name']))) {
    throw new HttpError(403, 'duplicated name');
  }

  return nameSpace.save();
}

export function getNameSpaceById(nameSpaceId: Types.ObjectId | string) {
  return NameSpace.findById(toObjectId(nameSpaceId, 'nameSpace'));
}

export async function updateNameSpace(
  nameSpaceId: string | Types.ObjectId,
  updater: AccountSchema,
  data: Partial<
    Pick<NameSpaceDoc, 'name' | 'title' | 'description'>
  >
) {
  const nameSpace = await getNameSpaceById(nameSpaceId);

  if (!nameSpace) {
    throw new HttpError(404, 'nameSpace not found');
  }

  nameSpace.set({ ...data, updater: updater.email, updateAt: new Date() });
  await nameSpace.validateAsync();
  return nameSpace.save();
}
