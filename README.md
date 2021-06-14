# Lookup

Makes it easy to perform DNS lookups. You can pipe a list to the tool or supply one or more subjects directly.

Usage:
```
USAGE:
    lookup [FLAGS] [OPTIONS] [subjects]...

FLAGS:
        --help       Prints help information
    -h, --host       host
    -i, --ip         ip
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    table or json output [env: LOOKUP_OUTPUT=]  [default: table]

ARGS:
    <subjects>...
```

## Reverse DNS lookup

```
❯ lookup --ip 173.0.84.110
+------+----------------------------+----------------+
| KIND | NAME                       | VALUE          |
+------+----------------------------+----------------+
| PTR  | 110.84.0.173.in-addr.arpa. | he.paypal.com. |
+------+----------------------------+----------------+
```

or

```
❯ cat ../ips.txt | ./target/debug/lookup --ip
+------+----------------------------+----------------+
| KIND | NAME                       | VALUE          |
+------+----------------------------+----------------+
| PTR  | 110.84.0.173.in-addr.arpa. | he.paypal.com. |
+------+----------------------------+----------------+
```

## DNS lookup

```
❯ lookup --host google.com amazon.com
+------+-------------+-----------------+
| KIND | NAME        | VALUE           |
+------+-------------+-----------------+
| A    | google.com. | 172.217.21.142  |
+------+-------------+-----------------+
| A    | amazon.com. | 54.239.28.85    |
+------+-------------+-----------------+
| A    | amazon.com. | 205.251.242.103 |
+------+-------------+-----------------+
| A    | amazon.com. | 176.32.103.205  |
+------+-------------+-----------------+
```

or 

```
❯ cat ../hosts.txt | ./target/debug/lookup --host
+------+-------------+-----------------+
| KIND | NAME        | VALUE           |
+------+-------------+-----------------+
| A    | google.com. | 142.250.74.110  |
+------+-------------+-----------------+
| A    | amazon.com. | 54.239.28.85    |
+------+-------------+-----------------+
| A    | amazon.com. | 205.251.242.103 |
+------+-------------+-----------------+
| A    | amazon.com. | 176.32.103.205  |
+------+-------------+-----------------+
```

## Failed lookup

```
❯ lookup --ip 104.21.85.250
+------+---------------+---------------+--------+
| KIND | NAME          | VALUE         | STATE  |
+------+---------------+---------------+--------+
|      | 104.21.85.250 | 104.21.85.250 | FAILED |
+------+---------------+---------------+--------+
```
