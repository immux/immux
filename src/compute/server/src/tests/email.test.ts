import { EMAIL_REGEX } from '@/constants';
import { expect } from 'chai';

test('`anchorgoogle@gmail.com` should match email', () => {
  expect('anchorgoogle@gmail.com').to.match(EMAIL_REGEX);
});

test('`anchorgoogle@163.com` should match email', () => {
  expect('anchorgoogle@163.com').to.match(EMAIL_REGEX);
});
