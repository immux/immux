import { Document, DocumentToObjectOptions, Types } from 'mongoose';
import { Assign } from 'utility-types';

export interface MarketFunctionsDoc {
  id: Types.ObjectId;

  name: string;

  projectId: string;

  title: string;

  description: string;

  creator: string;

  marketStatus: Boolean;

  price: number;

  updateAt: Date;
}

export interface MarketFunctionsSchema extends Document, Omit<MarketFunctionsDoc, 'id'> {
  toJSON(options?: DocumentToObjectOptions): MarketFunctionsDoc;

  validateAsync(): Promise<void>;
}
