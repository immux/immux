import * as Application from 'koa'
import { Context } from "koa";

export class Controller {
    ctx: Context;
    app: Application;
    constructor(ctx: Context) {
        this.ctx = ctx;
        this.app = ctx.app;
    }
}

