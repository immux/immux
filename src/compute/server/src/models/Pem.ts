import { PemSchema } from '@/types/models/Pem';
import { Schema, connection } from 'mongoose';

import { validateMongooseDocument } from '@/utils';
import { genJsonHandler } from '@/models/utils';
import _ = require('lodash');

const schema = new Schema<PemSchema>(
  {
    email: { type: String, required: true },

    hash: { type: String, required: true },

    publicPem: { type: String, required: true },

    privatePem: { type: String, required: true },

    createAt: { type: Number, required: true },

    rawText: { type: String },
  },
  { toJSON: genJsonHandler() }
);


schema.methods.validateAsync = validateMongooseDocument;

schema.index({ name: -1 }, { unique: true });

export default connection.model<PemSchema>('Pem', schema);
