import sys
import json
import hashlib

from extism import Plugin, Context, host_fn, Function, ValType
import sqlite3

@host_fn
def execute_sql(plugin, input, output, db_conn):
    # Get the SQL query from the input, it's raw utf-8 bytes
    mem = plugin.memory_at_offset(input[0])
    query = str(plugin.memory(mem)[:], 'UTF-8')

    # execute the query and serialize it as json bytes
    results = db_conn.execute(query).fetchall()
    out = json.dumps(results).encode()

    # allocate output memory
    mem = plugin.alloc(len(out))
    plugin.memory(mem)[:] = out
    output[0].value = mem.offset

if len(sys.argv) > 1:
    data = sys.argv[1].encode()
else:
    data = b"some data from python!"

def dict_factory(cursor, row):
    fields = [column[0] for column in cursor.description]
    return {key: value for key, value in zip(fields, row)}

with Context() as context:
    wasm = open("plugin/target/wasm32-unknown-unknown/release/plugin.wasm", 'rb').read()
    hash = hashlib.sha256(wasm).hexdigest()
    config = {"wasm": [{"data": wasm, "hash": hash}], "memory": {"max": 5}}

    # create read-only db connection and set as the host function user data
    db_conn = sqlite3.connect("file:accounts.db?mode=ro", uri=True)
    db_conn.row_factory = dict_factory

    functions = [
        Function(
            "execute_sql",
            [ValType.I64],
            [ValType.I64],
            execute_sql,
            db_conn,
        )
    ]

    plugin = context.plugin(config, functions=functions)

    input = json.dumps({"account_id": "bhelx"})
    result = plugin.call("on_event", input)
    print(result)

    input = json.dumps({"account_id": "nilslice"})
    result = plugin.call("on_event", input)
    print(result)

    input = json.dumps({"account_id": "zshipko"})
    result = plugin.call("on_event", input)
    print(result)

    input = json.dumps({"account_id": "unknown"})
    result = plugin.call("on_event", input)
    print(result)