import { Gender } from '@/constants';
import { Moment } from 'moment';

export interface Account {
  id: string;

  name: string;

  email: string;

  avatar: string;

  gender: Gender;

  createAt: Moment;
}

export interface NameSpace {
  id: string;

  name: string;

  title: string;

  description: string;

  creator: string;

  createAt: Moment;

  updater: string;

  updateAt: Moment;

  root: boolean;
}

/**
 * ComputedNameSpace
 * @description use in Store
 */
export interface ComputedNameSpace extends NameSpace {
  creator: Account;

  createAt: Moment | null;

  updater: Account;

  updateAt: Moment | null;
}

export interface MarketFunction {
  id: string;

  projectId: string;

  name: string;

  creator: string;

  price?: number;

  marketStatus?: boolean;

  description?: string;
}

export interface FileInfo {
  name: string,
  fileType: string,
  content: Buffer,
}
