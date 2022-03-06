# fast-conventional

Make conventional commits, faster, and consistently name scopes

## Usage

``` shell,script(name="help",expected_exit_code=0)
fast-conventional --help
```

``` text,verify(script_name="help",stream=stdout)
fast-conventional 1.1.1
Billie Thompson <billie@billiecodes.com>
Make conventional commits, faster, and consistently name scopes

USAGE:
    fast-conventional <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    completion    Generate completion for shell
    editor        Edit a commit message
    help          Print this message or the help of the given subcommand(s)
```

Completion

``` shell,script(name="help-completion",expected_exit_code=0)
fast-conventional completion --help
```

``` text,verify(script_name="help-completion",stream=stdout)
fast-conventional-completion 
Generate completion for shell

USAGE:
    fast-conventional completion <SHELL>

ARGS:
    <SHELL>    [possible values: bash, elvish, fish, powershell, zsh]

OPTIONS:
    -h, --help    Print help information
```

Editor

``` shell,script(name="help-completion",expected_exit_code=0)
fast-conventional editor --help
```

``` text,verify(script_name="help-completion",stream=stdout)
fast-conventional-editor 
Edit a commit message

USAGE:
    fast-conventional editor [OPTIONS] <COMMIT_MESSAGE_PATH>

ARGS:
    <COMMIT_MESSAGE_PATH>    The name of the file that contains the commit log message

OPTIONS:
    -c, --config <CONFIG>    Configuration file [env: FAST_CONVENTIONAL_CONFIG=] [default:
                             .fastconventional.yaml]
    -h, --help               Print help information
```


## Installing

See the [releases
page](https://github.com/PurpleBooth/fast-conventional/releases/latest)
we build for linux and mac (all x86_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/fast-conventional
```

This binary is designed to be run as a editor in git. To install it run

``` shell,skip()
git config --global alias.fci '-c "core.editor=\'fast-conventional editor\'" commit'
```

To trigger it when you commit run

``` shell,skip()
git fci
```

## Usage

> `.fastconventional.yaml`

``` yaml,file(path=".fastconventional.yaml")
use_angular: true
types: [ci]
scopes: ["mergify", "just", "github"]
```

``` text,file(path="commit.txt")
# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# On branch master
# Your branch is up to date with 'origin/master'.
#
# Changes to be committed:
#       new file:   README.md
```

We have a nice interactive UI.

![A terminal running the command
blow](demo.gif "A demo of the app running")

We can fake it using the below example simulates the steps

``` shell,script(name="full")
{
    sleep 1
    echo -ne "fix\r"
    sleep 1
    echo -ne "github\r"
    sleep 1
    echo -ne "Something that changed\r"
    sleep 1
    echo -ne "the subject goes here\r"
    sleep 1
    echo -ne "\r"
} | socat - EXEC:'fast-conventional editor commit.txt',pty,setsid,ctty
```

Now if we look at the commit

``` shell,script(name="cat-file")
cat commit.txt
```

``` text,verify(name="cat-file")
fix(github)!: the subject goes here


BREAKING CHANGE: Something that changed
```

Once you have an existing message, you can also edit it

``` shell,script(name="editing")
{
    sleep 1
    echo -ne "\r"
    sleep 1
    echo -ne "\r"
    sleep 1
    echo -ne "A better BC reason\r"
    sleep 1
    echo -ne "\r"
    sleep 1
    echo -ne "\r"
} | socat - EXEC:'fast-conventional editor commit.txt',pty,setsid,ctty
```

Now if we look at the commit

``` shell,script(name="cat-edited-file")
cat commit.txt
```

``` text,verify(name="cat-edited-file")
fix(github)!: the subject goes here


BREAKING CHANGE: A better BC reason
```
