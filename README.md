# fast-conventional

Make fast commits run quickly

## Usage

``` shell,script(name="help",expected_exit_code=0)
fast-conventional --help
```

``` text,verify(script_name="help",stream=stdout)
fast-conventional 0.1.0

Billie Thompson <billie@billiecodes.com>

A fast way to fill in conventional commit messages

USAGE:
    fast-conventional [OPTIONS] <commit-message-path> [ARGS]

ARGS:
    <commit-message-path>      The name of the file that contains the commit log message
    <commit-message-source>    The commit message, and can be: message (if a -m or -F option
                               was given to git); template (if a -t option was given or the
                               configuration option commit.template is set in git); merge (if
                               the commit is a merge or a .git/MERGE_MSG file exists); squash
                               (if a .git/SQUASH_MSG file exists); or commit
    <commit-sha>               Commit SHA-1 (if a -c, -C or --amend option was given to git).

OPTIONS:
    -c, --config <config>    Configuration file [env: FAST_CONVENTIONAL_CONFIG=]
                             [default: .fastconventional.yaml]
    -h, --help               Print help information
    -V, --version            Print version information
```


## Installing

See the [releases
page](https://github.com/PurpleBooth/fast-conventional/releases/latest) we
build for linux and mac (all x86_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/fast-conventional
```

## Usage

```yaml,file(path=".fastconventional.yaml")
use_angular: true
types: [ci]
scopes: ["abc", "bcd", "cde"]
```

```text,file(path="commit.txt")
# Some commit message template
```

```shell,script(name="full")
{
    sleep 1
    echo -ne "fix\r"
    sleep 1
    echo -ne "bcd\r"
    sleep 1
    echo -ne "Something that changed\r"
    sleep 1
    echo -ne "the subject goes here\r"
    sleep 1
    echo -ne "\r"
} | socat - EXEC:'fast-conventional commit.txt',pty,setsid,ctty
```

```shell,script(name="cat-file")
cat commit.txt
```

```text,verify(name="cat-file")
fix(bcd)!: the subject goes here


BREAKING CHANGE: Something that changed
```
