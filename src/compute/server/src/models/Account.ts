import { AccountSchema } from '@/types/models/Account';
import { Schema, connection } from 'mongoose';

import { genJsonHandler } from '@/models/utils';
import _ = require('lodash');
import { validateMongooseDocument } from '@/utils';

const schema = new Schema<AccountSchema>(
  {
    name: { type: String, required: true },

    email: { type: String, required: true, unique: true, index: true },

    avatar: { type: String, default: null },

    createAt: { type: Date, default: Date.now }
  },
  {
    toJSON: genJsonHandler((doc, ret) =>
      _.pick(ret, ['id', 'name', 'email', 'avatar', 'createAt'])
    )
  }
);

schema.methods.validateAsync = validateMongooseDocument;

export default connection.model<AccountSchema>('Account', schema);
