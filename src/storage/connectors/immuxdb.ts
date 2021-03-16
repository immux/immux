import {
    ImmuxDbCollection,
    ImmuxDbDocument,
    ImmuxDBFindCondition,
    ImmuxDbJS,
    ImmuxDbTransactions,
} from "./immuxdb.types";

interface UpdateRecordJS {
    height: number,
    value: string,
}

export interface ImmuxDBHttp {
    host: string;
    simpleGet(grouping: string, key: string): Promise<string>;
    select(grouping: string, condition: string): Promise<string>;
    inspect(grouping: string, key: string): Promise<UpdateRecordJS[]>;
    set(grouping: string, key: string, value: string): Promise<string>;
    revertOne(grouping: string, key: string, height: number): Promise<string>;
    revertAll(height: number): Promise<string>;
    deleteOne(grouping: string, key: string): Promise<string>;
    deleteAll(): Promise<string>;
    createTransactions(): Promise<string>;
    commitTransactions(tid: number): Promise<string>;
    abortTransactions(tid: number): Promise<string>;
    simpleTransactionsGet(tid: number, grouping: string, key: string): Promise<string>;
    transactionsSet(tid: number, grouping: string, key: string, value: string): Promise<string>;
    transactionsRevertOne(tid: number, grouping: string, key: string, height: number): Promise<string>;
    transactionsDeleteOne(tid: number, grouping: string, key: string): Promise<string>;
}

export function makeImmuxDBHttp(
    host: string,
    fetch: (path: string, options?: any) => Promise<any>
): ImmuxDBHttp {
    return {
        host,
        async simpleGet(grouping: string, key: string) {
            const response = await fetch(
                `http://${this.host}/${grouping}/${key}`
            );
            return await response.text();
        },
        // todo
        async select(grouping: string, condition: string) {
            const response = await fetch(
                `http://${this.host}/${grouping}/?select=${condition}`
            );
            return await response.text();
        },
        async inspect(grouping: string, key: string) {
            const response = await fetch(
                `http://${this.host}/${grouping}/${key}/.journal`
            );
            const text = await response.text();
            return text.split('\r\n')
                       .map((line: string) => line.split('|'))
                       .map((segments: string[]): UpdateRecordJS => ({
                           height: +segments[0],
                           value: segments[1]
                       }))
        },
        async set(grouping: string, key: string, value: string) {
            const response = await fetch(
                `http://${this.host}/${grouping}/${key}`,
                {
                    method: "PUT",
                    body: value
                }
            );
            return await response.text();
        },
        async revertOne(grouping: string, key: string, height: number) {
            const response = await fetch(
                `http://${this.host}/${grouping}/${key}?height=${height}`,
                {
                    method: "PUT"
                }
            );
            return await response.text();
        },
        async revertAll(height: number) {
            const response = await fetch(
                `http://${this.host}/?height=${height}`,
                {
                    method: "PUT"
                }
            );
            return await response.text();
        },
        async deleteOne(grouping: string, key: string) {
            const response = await fetch(
                `http://${this.host}/${grouping}/${key}`,
                {
                    method: "DELETE"
                }
            );
            return await response.text();
        },
        async deleteAll() {
            const response = await fetch(
                `http://${this.host}/}`,
                {
                    method: "DELETE"
                }
            );
            return await response.text();
        },
        async createTransactions() {
            const response = await fetch(
                `http://${this.host}/.transactions`,
                {
                    method: "POST"
                }
            );
            return await response.text();
        },
        async commitTransactions(tid: number) {
            const response = await fetch(
                `http://${this.host}/.transactions/${tid}?commit`,
                {
                    method: "POST"
                }
            );
            return await response.text();
        },
        async abortTransactions(tid: number) {
            const response = await fetch(
                `http://${this.host}/.transactions/${tid}?abort`,
                {
                    method: "POST"
                }
            );
            return await response.text();
        },
        async simpleTransactionsGet(tid: number, grouping: string, key: string) {
            const response = await fetch(
                `http://${this.host}/.transactions/${tid}/${grouping}/${key}`
            );
            return await response.text();
        },
        async transactionsSet(tid: number, grouping: string, key: string, value: string) {
            const response = await fetch(
                `http://${this.host}/.transactions/${tid}/${grouping}/${key}`,
                {
                    method: "PUT",
                    body: value
                }
            );
            return await response.text();
        },
        async transactionsRevertOne(tid: number, grouping: string, key: string, height: number) {
            const response = await fetch(
                `http://${this.host}/.transactions/${tid}/${grouping}/${key}?height=${height}`,
                {
                    method: "PUT"
                }
            );
            return await response.text();
        },
        async transactionsDeleteOne(tid: number, grouping: string, key: string) {
            const response = await fetch(
                `http://${this.host}/.transactions/${tid}/${grouping}/${key}`,
                {
                    method: "DELETE"
                }
            );
            return await response.text();
        },
    };
}

function getJsonReducer<T = any>(prev: T[], s: string): T[] {
    try {
        return [...prev, JSON.parse(s) as T];
    } catch {
        return prev;
    }
}

export function createImmuxDbViaHttpsRestrictedAccess(
    db: ImmuxDBHttp
): ImmuxDbJS {
    return new Proxy<ImmuxDbJS>(
        {},
        {
            get: (_, collectionName) => {
                const collectionObject: ImmuxDbCollection = {
                    upsert: async (doc: ImmuxDbDocument) => {
                        doc.id = doc.id || parseInt(Math.random().toString().slice(2)).toString();
                        await db.set(
                            collectionName.toString(),
                            doc.id,
                            JSON.stringify(doc)
                        );
                    },
                    find: async <T extends ImmuxDbDocument = ImmuxDbDocument>(
                        condition?: ImmuxDBFindCondition<T>
                    ) => {
                        const result = await db.select(
                            collectionName.toString(),
                            JSON.stringify(condition)
                        );
                        const rows = result.split("\r\n");
                        let data = rows.reduce<T[]>(getJsonReducer, []);
                        if (condition) {
                            data = data.filter(doc => {
                                for (const key in condition) {
                                    if (condition[key] !== doc[key]) {
                                        return false;
                                    }
                                }
                                return true;
                            });
                        }
                        return data;
                    },
                    findOne: async <
                        T extends ImmuxDbDocument = ImmuxDbDocument
                    >(
                        condition?: ImmuxDBFindCondition<T>
                    ) => {
                        const results = await collectionObject.find<T>(
                            condition
                        );
                        if (results[0]) {
                            return results[0];
                        } else {
                            return null;
                        }
                    },
                    deleteOne: async (id: string) => {
                        await db.deleteOne(
                            collectionName.toString(),
                            id
                        );
                    },
                    deleteAll: async () => {
                        await db.deleteAll();
                    },
                    revertOne: async (id: string, height: number) => {
                        await db.revertOne(
                            collectionName.toString(),
                            id,
                            height
                        );
                    },
                    revertAll: async (height: number) => {
                        await db.revertAll(height);
                    },
                    createTransactions: async () => {
                        // todo timeout机制 https://github.com/immux/immux/issues/15
                        const result = await db.createTransactions();
                        return Number(result);
                    },
                    commitTransactions: async (tid: number) => {
                        await db.commitTransactions(tid);
                    },
                    abortTransactions: async (tid: number) => {
                        await db.abortTransactions(tid);
                    },
                    simpleTransactionsGet: async (tid: number, id: string) => {
                        await db.simpleTransactionsGet(
                            tid,
                            collectionName.toString(),
                            id
                        );
                    },
                    transactionsUpsert: async (doc: ImmuxDbTransactions) => {
                        doc.id = doc.id || parseInt(Math.random().toString().slice(2)).toString();
                        await db.transactionsSet(
                            doc.tid,
                            collectionName.toString(),
                            doc.id,
                            JSON.stringify(doc)
                        );
                    },
                    transactionsRevertOne: async (tid: number, id: string, height: number) => {
                        await db.transactionsRevertOne(
                            tid,
                            collectionName.toString(),
                            id,
                            height,
                        );
                    },
                    transactionsDeleteOne: async (tid: number, id: string) => {
                        await db.transactionsDeleteOne(
                            tid,
                            collectionName.toString(),
                            id,
                        );
                    },
                };
                return collectionObject;
            }
        }
    );
}
