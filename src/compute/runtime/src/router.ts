import logger from "./logger";

const methods = ['get', 'post', 'patch', 'del', 'options', 'put']

interface Routers {
    [key: string]: Array<{
        httpMethod: string,
        constructor: any,
        handler: string
    }>
}
interface RouterInfo {
    httpMethod: string,
    constructor: any,
    handler: string
}
interface Decorator {
    (target: any, propertyKey: string): void
}

export interface RouterInstance extends Router {
    /**
     * http post method
     * @param url 
     */
    post(url: string): Decorator;

    /**
     * http get method
     * @param url 
     */
    get(url: string): Decorator;
    patch(url: string): Decorator;
    del(url: string): Decorator;
    options(url: string): Decorator;
    put(url: string): Decorator;
}

class Router {
    router: Routers = {}
    setRouter(url: string, routerInfo: RouterInfo) {
        const _currentRoutingGroup = this.router[url];
        if (_currentRoutingGroup) {
            for (const index in _currentRoutingGroup) {
                const object = _currentRoutingGroup[index];
                if (object.httpMethod === routerInfo.httpMethod) {
                    logger.error(`router path ${object.httpMethod} ${url} already exist`);
                    return
                }
            }
            this.router[url].push(routerInfo);
        } else {
            this.router[url] = [];
            this.router[url].push(routerInfo);
        }
    }

    getRoute() {
        return this.router;
    }
}

methods.forEach((httpMethod) => {
    Object.defineProperty(Router.prototype, httpMethod, {
        get: function () {
            return (url: string) => {
                return (target: any, propertyKey: string) => {
                    (<any>this).setRouter(url, {
                        httpMethod: httpMethod,
                        constructor: target.constructor,
                        handler: propertyKey
                    })
                }
            }
        }
    })
})

export const router: RouterInstance = <any>new Router
