import { Controller } from '../../src/base/controller';
import { router } from '../../src/router';

export default class Example extends Controller {
    @router.get('/example')
    async getExample() {
        this.ctx.body = 'example get'
    }

    @router.get('/news')
    async getNews() {
        this.ctx.body = 'news info get'
    }

    @router.post('/example')
    async postExample() {
        this.ctx.body = JSON.stringify(this.ctx.request.body);
    }
}
