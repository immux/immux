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
    async transactionalGet(key, tid) {
        const {stdout, stderr} = execAsync(`cargo run get "${key}" ${tid}`);
        return extractOk(stdout);
    },

    async transactionalSet(key, value, tid) {
        return execAsync(`cargo run set "${key}" "${value}" ${tid}`);
    },
}

async function test_simple_getset() {
    const key = "key";
    const value = "VALUE";
    await db.set(key, value);
    assert.equal(await db.get(key), value);
}

async function test_multiple_sets() {
    const inserts = 100;
    // const inserts = 1e8;
    for (let i = 0; i < inserts; i++) {
        await db.set(i, "Value");
    }
    assert.equal(await db.get(0), "Value");
}

async function test_set_large_key() {
    const key = Array.from(Array(1024 * 8)).fill("X").join("");
    const value = "value";
    await db.set(key, value);
    const valueOut = await db.get(key);
    assert.equal(value, valueOut);
}

async function test_set_key_too_large() {
    const key = Array.from(Array(1024 * 10)).fill("X").join("");
    await assert.rejects(async () => await db.set(key, ""));
}

async function test_dirty_read_from_nontransactional_context() {
    const key = "key";
    const value = "VALUE";
    const tid = await db.startTransaction();
    await db.transactionalSet(key, value, tid);
    assert.equal(await db.get(key), null); // Not yet committed
    await db.commitTransaction(tid);
    assert.equal(await db.get(key), value); // Committed
}

async function test_dirty_read_from_transactional_context() {
    const key = "key";
    const value = "VALUE";
    const tidWriter = await db.startTransaction();
    const tidReader = await db.startTransaction();

    await db.transactionalSet(key, value, tidWriter);

    const readerViewBeforeCommit = await db.transactionalGet(key, tidReader);
    assert.equal(readerViewBeforeCommit, null); // Writer not yet committed

    await db.commitTransaction(tidWriter);
    const readerViewAfterCommit = await db.transactionalGet(key, tidReader);
    assert.equal(readerViewAfterCommit, null); // Writer committed, but still shouldn't be visible to reader
}

async function test_simple_abort() {
    const key = "K";
    const originalValue = "1";
    const abortedValue = "2";
    await db.set(key, originalValue);

    const tid = await db.startTransaction();
    await db.transactionalSet(key, abortedValue, tid);
    await db.abortTransaction(tid);
    const finalValue = await db.get(key)

    assert.equal(finalValue, originalValue);
}

async function test_operate_on_aborted() {
    const tid = await db.startTransaction();
    await db.abortTransaction(tid);
    await assert.rejects(async () => await db.transactionalSet("A", "B", tid));
}

async function test_abort_nonexistent() {
    await assert.rejects(async () => await db.abortTransaction(Math.random()));
}

async function test_read_history_from_transaction() {
    const key = "KEY";
    const value = "VALUE";

    await db.set(key, value);
    const tid = await db.startTransaction();
    const valueOut = await db.transactionalGet(key, tid);

    assert.equal(value, valueOut);
}

async function test_nonrepeatable_read() {
    const key = "KEY"
    const originalValue = "original";
    await db.set(key, originalValue);
    const tidA = await db.startTransaction();
    const tidB = await db.startTransaction();
    await db.transactionalSet(key, "new value", tidA);
    await db.commitTransaction(tidA);

    const valueInTransactionB = await db.transactionalGet(key, tidB);
    assert.equal(valueInTransactionB, originalValue);
}

async function test() {
    const tests = [
        test_simple_getset,
        test_multiple_sets,
        test_set_large_key,
        test_set_key_too_large,
        test_dirty_read_from_nontransactional_context,
        test_dirty_read_from_transactional_context,
        test_simple_abort,
        test_operate_on_aborted,
        test_abort_nonexistent,
        test_read_history_from_transaction,
        test_nonrepeatable_read,
    ]
    let allPassed = true;
    for (const test of tests) {
        process.stdout.write(`Testing ${test.name}...`);
        await db.reset();
        try {
            await test();
            process.stdout.write("OK\n")
        }
        catch (error) {
            process.stdout.write("ERROR! \n")
            console.error(error);
            allPassed = false;
        }
    }
    if (allPassed) {
        console.log("All tests passed.");
    }
}

test();
