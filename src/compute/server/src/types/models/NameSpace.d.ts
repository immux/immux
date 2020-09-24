import { Document, DocumentToObjectOptions, Types } from 'mongoose';
import { Assign } from 'utility-types';

export interface NameSpaceDoc {
  id: Types.ObjectId;

  readonly name: string;

  title: string;

  description: string;

  creator: string;

  createAt: Date;

  updater: string;

  updateAt: Date;
}

export interface NameSpaceSchema extends Document, Omit<NameSpaceDoc, 'id'> {
  toJSON(options?: DocumentToObjectOptions): NameSpaceDoc;

  validateAsync(): Promise<void>;
}
