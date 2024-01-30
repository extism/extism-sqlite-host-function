# Extism Host Function SQL Demo

> Warning: This example is pre-1.0 Extism. If you'd like to update it, please do! If you're not sure how
> please reach out to us in Discord.

This is a simple demo of using a host function to expose a SQL database to your plugin. In this case our host
is python and our database is a sqlite database. Our plugin is written in Rust.

## Build and Run

```bash
poetry install
cd plugin/ && cargo build --target wasm32-unknown-unknown --release && cd ..
poetry run python main.py
```

## How it works

The host holds a read-only connection to the sqlite database of accounts:

```python
db_conn = sqlite3.connect("file:accounts.db?mode=ro", uri=True)
```

We can create a host function called `execute_sql` that allows our plugin to execute sql on it.

> **Note**: If you want more control or security, you should consider giving your plugin a more granular interface instead of letting it make raw queries.


```python
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
```

The `execute_sql` function takes the db_conn as userdata so it can be used by the implementation. You can think of this like a closure. The function signature has a pointer (i64) input and a pointer (i64) output.

* The `input` is a pointer to a UTF-8 SQL string
* The `output` is a pointer to a UTF-8 JSON string of results

We can implement it like this:


```python
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
```



