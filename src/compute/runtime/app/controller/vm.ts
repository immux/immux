import * as fs from 'fs'
import * as path from 'path'
import * as vm from 'vm'

import { Controller } from '../../src/base/controller';
import { router } from '../../src/router';
import { randomBytes, scrypt, createHmac } from 'crypto';

interface VmOptions {
    id: string;
}
interface FnObject {
    [key: string]: string;
}
  
interface ReadFilesParams {
    dirname: string;
    onError: (options: string) => void;
}
interface FnsResult {
    [key: string]: () => any;
}
interface Sandbox {
    [key: string]: any;
}

  
const TIMEOUT = 1000 * 1.5 * 10000;

export default class Runtime extends Controller {
    fnPool: FnObject = {}

    onFileContent(filename: string, content: string): void {
        this.fnPool[filename.replace(/\.ts|\.js/g, '')] = content;
    }

    async readFiles({ dirname, onError }: ReadFilesParams): Promise<FnsResult> {
        return new Promise(next => {
          fs.readdir(dirname, (err: NodeJS.ErrnoException | null, filenames: string[]) => {
            if (err) {
              onError(err.message);
              return;
            }
    
            let sum = 0;
    
            filenames.forEach(filename => {
              fs.readFile(
                `${dirname}/${filename}`,
                'utf-8',
                (err: NodeJS.ErrnoException | null, content: string) => {
                  if (err) {
                    onError(err.message);
                    return;
                  }
                  this.onFileContent(filename, content);
                  sum++;
    
                  if (sum === filenames.length) {
                    next();
                  }
                },
              );
            });
          });
        });
    }

    async getFn(options: VmOptions): Promise<string> {
        await this.readFiles({
            dirname: path.resolve('./app/fns'),
            onError: err => console.error(err),
        });

        const fnString = this.fnPool[options.id];

        return fnString;
    }

    @router.get('/vm/:project/:fn')
    @router.get('/vm/:project/:fn/:param')
    @router.post('/vm/:project/:fn')
    @router.post('/vm/:project/:fn/:param')
    @router.del('/vm/:project/:fn')
    @router.del('/vm/:project/:fn/:param')
    async index() {
        let timer = null;
        const split: string = this.ctx.params.project;
        const path: string = this.ctx.request.url.split(split)[1];

        // An entry function
        let fnString = await this.getFn({ id: 'main' });

        // JSON.stringify drop constructor
        let ctx = JSON.stringify(this.ctx);
        let body = JSON.stringify(this.ctx.request.body);

        const db = JSON.stringify({});
        const provider = JSON.stringify({ info: 'function', path});

        // Load purchased functions, plugins
        const fns = JSON.stringify({});
        const plugins = JSON.stringify({});

        fnString = `${fnString}
            const ctx = {
                provider: ${provider},
                content: ${ctx},
                fns: ${fns},
                plugins: ${plugins},
                body: ${body},
            }
            const db = ${db};

            main(ctx, db)
        ;`;

        const result = await new Promise(
            (next: (data: any) => void, reject: (err: Error) => void) => {
                const sandbox: Sandbox = {
                    setInterval,
                    setTimeout,
                    crypto: {
                        randomBytes,
                        scrypt,
                        createHmac,
                    },
                    // Using global isn't consistent across different node versions: e.g. Buffer is in global for node v8 and v10 but not v12.
                    Buffer,
                    require,
                    console,
                    requirePath: process.cwd(),
                    // moment,
                };
        
              console.time('vm'); 
      
              try {
                // Debug to remove the timeout
                // timr = setTimeout(() => {
                //   reject(new Error('Script execution timed out.'));
                // }, TIMEOUT);
              
                vm.createContext(sandbox);

                // load config，get pre-hook and run it

                const data = vm.runInContext(fnString, sandbox, {
                //  filename: id,
                    timeout: TIMEOUT,
                });
        
                console.timeEnd('vm');

                // load config，get after-hook and run it
        
                next(data);
              } catch (error) {
                reject(error);
              }
            },
          ).catch(err => {
              return err instanceof Error ? err : new Error(err.stack);
          });
      
          if (timer) {
              clearTimeout(timer);
              timer = null;
          }
      
          let resBody = {};
      
          if (result instanceof Error) {
              console.log('[ERROR]', result);
      
              resBody = {
                  error: result.toString
                    ? result.toString().replace(/Error: Error: /g, 'Error: ')
                    : result,
              };
          } else {
              console.log('[Response]', result);
      
              resBody = result;
          }
      
          this.ctx.body = resBody;
    }
}
