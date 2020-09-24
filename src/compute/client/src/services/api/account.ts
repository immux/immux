import { createInstance } from '@/utils/axios';
import { Account } from '@/types/models';

const instance = createInstance('/api/account');

/**
 * get current account profile
 */
export function fetchCurrentAccountProfile() {
  return instance.get<Account, Account>('');
}

/**
 * get current account pems
 */
export function fetchCurrentAccountPems() {
  return instance.get<
    any,
    {
      hash: string;
      email: string;
      privatePem?: string;
      publicPem: string;
      createAt: number;
    }
  >('/pems');
}

/**
 * login api
 * @param code `code`
 * @param redirectUri
 */
export function login(code: string, redirectUri: string) {
  return instance.post<
    any,
    {
      accessToken: string;
      expiresIn: number;
      account: Account;
    }
  >('/login', {
    code,
    redirectUri
  });
}

/**
 * logout api
 */
export function logout() {
  return instance.post<any, string>('/logout');
}

/**
 * create pems
 */
export function createAccountPems() {
  return instance.post<
    any,
    {
      hash: string;
      email: string;
      publicPem: string;
      createAt: number;
      privatePem?: string;
    }
  >('/pems');
}

/**
 * destroy pems
 */
export function destroyAccountPems() {
  return instance.delete<any, { message: 'ok' }>('/pems');
}
