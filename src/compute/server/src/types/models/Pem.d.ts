import { Document, DocumentToObjectOptions, Types } from 'mongoose';

export interface PemDoc {
  id: Types.ObjectId;

  readonly email: string;

  hash: string;

  publicPem: string;

  privatePem: string;

  createAt: number;

  rawText: string;
}

export interface PemSchema extends Document, Omit<PemDoc, 'id'> {
  toJSON(options?: DocumentToObjectOptions): PemDoc;

  validateAsync(): Promise<void>;
}
