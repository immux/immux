import { Document, DocumentToObjectOptions, Types } from 'mongoose';
import { Gender } from '@/constants';

/**
 * 账号文档
 */
export interface AccountDoc {
  /**
   * 编号
   */
  id: Types.ObjectId;

  /**
   * 姓名
   */
  name: string;

  /**
   * 邮箱
   */
  email: string;

  /**
   * 头像
   */
  avatar: string;

  /**
   * 性别
   */
  gender: Gender;

  /**
   * 创建时间
   */
  createAt: Date;
}

/**
 * 账号概要
 */
export interface AccountSchema extends Document, Omit<AccountDoc, 'id'> {
  toJSON(
    options?: DocumentToObjectOptions
  ): Pick<
    AccountDoc,
    'id' | 'name' | 'email' | 'avatar' | 'gender' | 'createAt'
  >;

  /**
   * 校验模型
   */
  validateAsync(): Promise<void>;
}
