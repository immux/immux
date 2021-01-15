import { MarketFunctionsSchema } from '@/types/models/MarketFunctions';
import { Schema, connection } from 'mongoose';

import { validateMongooseDocument } from '@/utils';
import { genJsonHandler } from '@/models/utils';
import _ = require('lodash');

const schema = new Schema<MarketFunctionsSchema>(
  {
    name: { type: String, required: true },

    projectId: { type: String, required: true },

    title: { type: String, trim: true, default: null },

    description: { type: String, trim: true, default: null },

    creator: { type: String, required: true },

    marketStatus: { type: Boolean, default: true },

    price: { type: Number, default: 0 },

    updateAt: { type: Date, default: null },
  },
  { toJSON: genJsonHandler() }
);


schema.methods.validateAsync = validateMongooseDocument;

export default connection.model<MarketFunctionsSchema>('MarketFunctions', schema);
