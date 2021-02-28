#!/usr/bin/env node

require('module-alias').addAliases({ '@': __dirname });

import program = require('commander');
import upload from '@/upload';
import figlet from 'figlet';
import chalk from 'chalk';

const pkgJson = require('../package.json');

program.version(pkgJson.version).usage('<command> [options]');

// upload
program.command('upload').description('upload project').action(upload);

// other command

program.parse(process.argv);

if (!program.args.length) {
  console.log(
    figlet.textSync('COMPUTE CLI', {
      font: 'Small Slant'
    })
  );

  console.log(chalk.gray(`(${pkgJson.version})`), '\n');
  program.help();
}
