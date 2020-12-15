import { NameSpaceSchema } from '@/types/models/NameSpace';
import { Schema, connection } from 'mongoose';

import { isValidName } from '@/services';
import { validateMongooseDocument } from '@/utils';
import { genJsonHandler } from '@/models/utils';
import _ = require('lodash');

const schema = new Schema<NameSpaceSchema>(
  {
    name: { type: String, required: true, validate: isValidName },

    title: { type: String, trim: true, default: null },

    description: { type: String, trim: true, default: null },

    creator: { type: String, required: true },

    createAt: { type: Date, default: Date.now },

    updater: { type: String, default: null },

    updateAt: { type: Date, default: null },
  },
  { toJSON: genJsonHandler() }
);


schema.methods.validateAsync = validateMongooseDocument;

schema.index({ name: -1 }, { unique: true });

export default connection.model<NameSpaceSchema>('NameSpace', schema);
