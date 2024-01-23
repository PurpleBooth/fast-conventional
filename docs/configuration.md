# Configuration

We have a useful tool for outputting an example configuration tool

## Binary Usage

``` shell,script(name="help-example",expected_exit_code=0)
fast-conventional example-config --help
```

``` text,verify(script_name="help-example",stream=stdout)
Print an example configuration

Usage: fast-conventional example-config

Options:
  -h, --help  Print help
```

To generate the example

``` shell,script(name="example-config")
fast-conventional example-config
```

## About the file

The file looks like this. All fields are optional.

``` yaml,verify(name="example-config")
use_angular: true
require_scope: null
types:
- custom_type
scopes:
- src
- actions
- manpages
- readme
- e2e
- unit

```

The "use_angular" option will save you some typing by including angular
types automatically.

So this

``` yaml,skip()
---
use_angular: true
```

is equivilent to

``` yaml,skip()
types:
  - feat
  - fix
  - docs
  - style
  - refactor
  - perf
  - test
  - chore
  - build
  - ci
```
