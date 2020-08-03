export type ImmuxDBFindCondition<T> = { [key in keyof T]?: T[key] };

export interface ImmuxDbDocument {
    id?: number;
}

export interface ImmuxDbTransactions {
    tid: number;
    id?: number;
}

export interface ImmuxDbCollection {
    upsert: (doc: ImmuxDbDocument) => Promise<void>;
    find: <T extends ImmuxDbDocument = ImmuxDbDocument>(
        condition?: ImmuxDBFindCondition<T>
    ) => Promise<T[]>;
    findOne: <T extends ImmuxDbDocument = ImmuxDbDocument>(
        condition?: ImmuxDBFindCondition<T>
    ) => Promise<T | null>;
    deleteOne: (id: number) => Promise<void>;
    deleteAll: () => Promise<void>;
    revertOne: (id: number, height: number) => Promise<void>;
    revertAll: (height: number) => Promise<void>;
    createTransactions: () => Promise<number>;
    commitTransactions: (tid: number) => Promise<void>;
    abortTransactions: (tid: number) => Promise<void>;
    simpleTransactionsGet: (tid: number, id: number) => Promise<void>;
    transactionsUpsert: (doc: ImmuxDbTransactions) => Promise<void>;
    transactionsRevertOne: (tid: number, id: number, height: number) => Promise<void>;
    transactionsDeleteOne: (tid: number, id: number) => Promise<void>;
}

export type ImmuxDbJS = { [collection in string]: ImmuxDbCollection };
