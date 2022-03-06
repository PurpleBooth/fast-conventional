# Editor

Mostly users don't need to interact directly with the editor, from a
commandline perspective

## Binary Usage

``` shell,script(name="help-editor",expected_exit_code=0)
fast-conventional editor --help
```

``` text,verify(script_name="help-editor",stream=stdout)
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

## Conventional Commits

Given we have configured the tool, it looks for this in the root of the
git repository.

> `.fastconventional.yaml`

``` yaml,file(path=".fastconventional.yaml")
use_angular: true
types: [ci]
scopes: ["mergify", "just", "github"]
```

When we commit, git has generated this stub configuration

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

We can add our conventional message using this neat UI

![A terminal running the command
blow](../demo.gif "A demo of the app running")

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

## Non-conventional commits

For commits that are not conventional, but already have some body text
in them, we will display a prompt saying it's not conventional, and
asking the user if they want to open the default editor.

## Installing

Once the binary is installed use this command to cofigure it. You could
change the alias if you prefer. I chose fci, as it is similar to the
"ci" alias, many people use.

``` shell,skip()
git config --global alias.fci '-c "core.editor=fast-conventional editor" commit'
```
