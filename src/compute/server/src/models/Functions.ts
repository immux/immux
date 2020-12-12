import { FunctionsSchema } from '@/types/models/Functions';
import { Schema, connection } from 'mongoose';

import { validateMongooseDocument } from '@/utils';
import { genJsonHandler } from '@/models/utils';
import _ = require('lodash');

const schema = new Schema<FunctionsSchema>(
  {
    name: { type: String, required: true },

    projectId: { type: String, required: true },

    description: { type: String, trim: true, default: null },

    creator: { type: String, required: true },

    marketStatus: { type: String, default: false },

    price: { type: Number, default: 0 },

    updateAt: { type: Date, default: null },
  },
  { toJSON: genJsonHandler() }
);


schema.methods.validateAsync = validateMongooseDocument;

schema.index({ name: -1 }, { unique: true });

export default connection.model<FunctionsSchema>('Functions', schema);
