# jsonset
Set JSON values in JSON file by JSONPath.

Command line usage:
```text
USAGE:
    jsonset [FLAGS] <FILE> --key <KEY> --value <VALUE>

FLAGS:
    -b, --as-bool      set value as bool
    -u, --as-null      set value as null
    -n, --as-number    set value as number
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -k, --key <KEY>        the JSONPath to select nodes
    -v, --value <VALUE>    the value to set to

ARGS:
    <FILE>    the file containing json text
```

For example, with a JSON file provided as below:
```json
{
  "hello": "test",
  "world": "check"
}
```

Running following:
```text
jsonset test.json --key $.hello --value example
```
will update the file content to:
```json
{
  "hello": "example",
  "world": "check"
}
```

Running following:
```text
jsonset test.json --key $.hello --value 32.14 --as-number
```
will update the file content to:
```json
{
  "hello": 32.14,
  "world": "check"
}
```