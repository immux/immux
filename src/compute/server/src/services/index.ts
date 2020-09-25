import { NAME_REGEX, ROOT_NAME } from '@/constants';

export function isValidName(name: string) {
  return NAME_REGEX.test(name) || name === ROOT_NAME;
}

export function isRootName(name: string) {
  return name === ROOT_NAME;
}
