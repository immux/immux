import { Document, DocumentToObjectOptions, Types } from 'mongoose';
import { Assign } from 'utility-types';

export interface FunctionsDoc {
  id: Types.ObjectId;

  name: string;

  projectId: string;

  description: string;

  creator: string;

  marketStatus: Boolean;

  price: number,

  updateAt: Date;
}

export interface FunctionsSchema extends Document, Omit<FunctionsDoc, 'id'> {
  toJSON(options?: DocumentToObjectOptions): FunctionsDoc;

  validateAsync(): Promise<void>;
}
