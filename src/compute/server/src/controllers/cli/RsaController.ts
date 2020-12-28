import {
  JsonController,
  BodyParam,
  HttpError,
  Post
} from 'routing-controllers';

import { PemSchema } from '@/types/models/Pem';
import { getPems, getRsaRawText, saveRsaRawText } from '@/services/rsa';
import { genAccessTicket, getAccountByEmail } from '@/services/account';
import { encryptRawText, genRawText } from '@/utils/rsa';
import { isDevelopment } from '@/constants';
import _ = require('lodash');

@JsonController('/cli/rsa')
export default class RsaController {
  @Post('/signature')
  async genSignature(
    @BodyParam('email') email: string,
    @BodyParam('publicPem') publicPem: string
  ) {
    const pems = await getPems(email, publicPem);
    const rawText = genRawText();
    
    const signature = encryptRawText(pems.privatePem, rawText);

    await saveRsaRawText(email, publicPem, signature, rawText);

    return isDevelopment ? { rawText, signature } : { signature };
  }

  @Post('/signature/verify')
  async verifySignature(
    @BodyParam('email') email: string,
    @BodyParam('publicPem') publicPem: string,
    @BodyParam('signature') signature: string,
    @BodyParam('rawText') rawText: string
  ) {
    if (!_.isString(signature)) {
      throw new HttpError(400, 'invalid signature');
    }

    if (!_.isString(rawText)) {
      throw new HttpError(400, 'invalid rawText');
    }

    const account = await getAccountByEmail(email);

    if (!account) {
      throw new HttpError(404, 'account not found');
    }

    const pem: PemSchema = await getRsaRawText(email, publicPem, signature);

    if (rawText !== pem.rawText) {
      throw new HttpError(403, 'rawText not match');
    }

    return _.defaults(await genAccessTicket(email), { account });
  }
}
