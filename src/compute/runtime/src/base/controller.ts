import { Frame } from "../core";
import { BaseContext } from "koa";



export class Controller {
    ctx: BaseContext;
    app: Frame;
    constructor(ctx: BaseContext) {
        this.ctx = ctx;
        //@ts-ignore
        this.app = ctx.app;
    }
}

