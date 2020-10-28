import { Controller } from '../../src/base/controller';
import { bp } from '../../src/blueprint';

export default class example extends Controller {
    @bp.get('/example')
    async getExample() {
        this.ctx.body = 'example get'
    }

    @bp.get('/news')
    async getNews() {
        this.ctx.body = 'news info get'
    }

    @bp.post('/example')
    async postExample() {
        //@ts-ignore
        this.ctx.body = JSON.stringify(this.ctx.request.body);
    }
}
