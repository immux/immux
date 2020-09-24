import NodeRSA = require('node-rsa');

export function decryptSignature(
  publicPem: string | Buffer,
  encryptedText: string
) {
  return new NodeRSA(publicPem).decryptPublic(encryptedText, 'utf8');
}
