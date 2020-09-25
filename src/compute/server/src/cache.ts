export enum CacheKeys {
  AccessToken,

  AccountPems,

  RsaRawText,

  AccessTicket
}

const generators = {
  [CacheKeys.AccessToken](emailOrToken: string) {
    return `account:${emailOrToken}:access-token`;
  },

  [CacheKeys.AccountPems](email: string) {
    return `account:${email}:pems`;
  },

  [CacheKeys.RsaRawText](email: string, publicPem: string, signature: string) {
    return `account:${email}:public-pem:${publicPem}:${signature}`;
  },

  [CacheKeys.AccessTicket](email: string) {
    return `ticket:${email}`;
  }
};

export default function genCacheKey<T extends CacheKeys>(cacheKey: T) {
  return generators[cacheKey];
}
