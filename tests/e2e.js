const fs = require("fs");
const util = require("util");
const assert = require("assert").strict;

const execAsync = util.promisify(require("child_process").exec);
const unlinkAsync = util.promisify(fs.unlink);

function extractOk(stdout) {
    const match = /Ok\("(.*)"\)/.exec(stdout);
    if (match) {
        return match[1];
    }
    else if (stdout === "Key not existed") {
        return null;
    }
    else {
        return null;
    }
}

const db = {
    async reset() {
        await unlinkAsync(`/tmp/command_log.log`)
    },
    async get(key) {
        const {stdout, stderr} = await execAsync(`cargo run get ${key}`);
        return extractOk(stdout);
    },
    async set(key, value) {
        return execAsync(`cargo run set "${key}" "${value}"`);
    },
    async startTransaction() {
        const {stdout, stderr} = await execAsync(`cargo run start_transaction`);
        return stdout;
    },
    async commitTransaction(tid) {
        return execAsync(`cargo run commit_transaction ${tid}`);
    },
    async abortTransaction(tid) {
        return execAsync(`cargo run abort_transaction ${tid}`);
    },
    async transactional_get(key, tid) {
        const {stdout, stderr} = execAsync(`cargo run get "${key}" ${tid}`);
        return extractOk(stdout);
    },

    async transactional_set(key, value, tid) {
        return execAsync(`cargo run set "${key}" "${value}" ${tid}`);
    },
}

async function test_simple_getset() {
    const key = "key";
    const value = "VALUE";
    await db.set(key, value);
    assert.equal(await db.get(key), value);
}

async function test_massive_set() {
    const inserts = 10;
    // const inserts = 1e8;
    for (let i = 0; i < inserts; i++) {
        await db.set(i, "Value");
    }
    assert.equal(await db.get(0), "Value");
}


async function test_dirty_read_from_nontransactional_context() {
    const key = "key";
    const value = "VALUE";
    const tid = await db.startTransaction();
    await db.transactional_set(key, value, tid);
    assert.equal(await db.get(key), null); // Not yet committed
    await db.commitTransaction(tid);
    assert.equal(await db.get(key), value); // Committed
}

async function test_dirty_read_from_transactional_context() {
    const key = "key";
    const value = "VALUE";
    const tidWriter = await db.startTransaction();
    const tidReader = await db.startTransaction();

    await db.transactional_set(key, value, tidWriter);

    const readerViewBeforeCommit = await db.transactional_get(key, tidReader);
    assert.equal(readerViewBeforeCommit, null); // Writer not yet committed

    await db.commitTransaction(tidWriter);
    const readerViewAfterCommit = await db.transactional_get(key, tidReader);
    assert.equal(readerViewAfterCommit, null); // Writer committed, but still shouldn't be visible to reader
}

async function test() {
    const tests = [
        test_simple_getset,
        test_massive_set,
        test_dirty_read_from_nontransactional_context,
        test_dirty_read_from_transactional_context,
    ]
    for (const test of tests) {
        await db.reset();
        await test();
    }
    console.log("All tests passed.")
}

test();
