import { PREFIX } from '@/constants';
import { Ora } from 'ora';

import { getAccessTicket, getPublicPem } from '@/services/rsa';
import { uploadFile } from '@/services/api';

import ora = require('ora');
import chalk from 'chalk';
import _ from 'lodash';
export class Uploader {
  readonly spinner: Ora;

  // Work list
  private readonly cwd: string;

  private email!: string;

  private publicPem!: string;

  private ticket!: string;

  private url!: string;

  constructor(cwd: string, spinner?: Ora) {
    this.spinner = spinner || ora();
    this.cwd = cwd;
  }

  async init() {
    await this.getPublicPem();
    // await this.getTicket();
  }

  async exec() {
    this.spinner.info(
      `${PREFIX} The system has started and the current working directory is ${chalk.gray(
        this.cwd
      )}`
    );

    await this.init();

    await this.uploadFiles();

    this.spinner.info(`${PREFIX} File task completed`);
  }

  async getPublicPem() {
    this.spinner.start(`${PREFIX} Obtaining public key certificate`);

    const { email, publicPem } = await getPublicPem();

    this.email = email;
    this.publicPem = publicPem;

    this.spinner.succeed(
      `${PREFIX} Successfully obtain the public key certificate`
    );
  }

  async getTicket() {
    this.spinner.start(`${PREFIX} In exchange for access ticket`);

    const { ticket, account } = await getAccessTicket(
      this.email,
      this.publicPem
    );

    this.ticket = ticket;

    this.spinner.succeed(
      `${PREFIX} ${chalk.greenBright(
        account.name
      )} Successfully exchanged ${chalk.gray('Access ticket')}`
    );
  }

  async uploadFiles() {
    await uploadFile();

    this.spinner.succeed(`${PREFIX} Successfully uploaded files`);
  }
}
