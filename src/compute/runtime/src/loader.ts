import * as KoaRouter from 'koa-router';
import * as fs from 'fs';
import logger from './logger';
import { BaseContext } from 'koa';
import { Frame } from './core';
import { router } from './router';
interface FileModule {
    module: any,
    filename: string
}

interface StringSub {
    source: string,
    isFound: boolean
}
interface Plugin {
    enable: boolean,
    package: string
}

export class Loader {
    private koaRouter: any = new KoaRouter;
    private app: Frame;

    constructor(app: Frame) {
        this.app = app;
    }

    private appDir() {
        const subString = removeString(__dirname, 'node_modules');
        if (subString.isFound) {
            return subString.source;
        }
        return subString.source.substr(0, subString.source.length - 4) + '/';
    }

    private fileLoader(url: string): Array<FileModule> {
        const merge = this.appDir() + url;

        return fs.readdirSync(merge).map((name) => {
            return {
                module: require(merge + '/' + name).default,
                filename: name
            };
        });
    }

    loadController() {
        this.fileLoader('app/controller');
    }

    loadRouter() {
        const r = router.getRoute();
        Object.keys(r).forEach((url) => {
            r[url].forEach((object) => {
                this.koaRouter[object.httpMethod](url, async (ctx: BaseContext) => {
                    const instance = new object.constructor(ctx);
                    await instance[object.handler]();
                })
            })
        })
        this.app.use(this.koaRouter.routes());
    }

    loadMiddleware() {
        try {
            const middleware = this.fileLoader('app/middleware');
            const registedMid = this.app.config['middleware'];

            if (!registedMid) return;
            registedMid.forEach((name: string) => {
                logger.blue(name);
                for (const index in middleware) {
                    const mod = middleware[index];
                    const fname = mod.filename.split('.')[0];
                    if (name === fname) {
                        this.app.use(mod.module());
                    }
                }
            })
        } catch (e) { }
    }

    loadConfig() {
        const configDef = this.appDir() + 'app/config/config.default.js';
        const configEnv = this.appDir()
            + (process.env.NODE_ENV === 'production' ? 'app/config/config.pro.js' : 'app/config/config.dev.js');
        const conf = require(configEnv).default;
        const confDef = require(configDef).default;
        const merge = Object.assign({}, conf, confDef);
        Object.defineProperty(this.app, 'config', {
            get: () => {
                return merge
            }
        })
    }

    loadPlugin() {
        const Pdir = this.appDir() + 'app/config/plugin.js';
        const plugins = require(Pdir).default;
        for (const index in plugins) {
            const plugin: Plugin = plugins[index];
            if (plugin.enable) {
                const pkg = require(plugin.package);
                pkg(this.app);
            }
        }
    }

    load() {
        this.loadConfig();
        this.loadPlugin();
        this.loadController();
        this.loadMiddleware();
        this.loadRouter();
    }
}

function removeString(source: string, str: string): StringSub {
    const index = source.indexOf(str);
    if (index > 0) {
        return {
            source: source.substr(0, index),
            isFound: true
        };
    }
    return {
        source: source,
        isFound: false
    };
}