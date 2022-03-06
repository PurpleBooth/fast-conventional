# fast-conventional

Make conventional commits, faster, and consistently name scopes

## Usage

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
blow](demo.gif "A demo of the app running")

## Installing

See the [releases
page](https://github.com/PurpleBooth/fast-conventional/releases/latest)
we build for linux and mac (all x86_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/fast-conventional
```

This binary is designed to be run as a editor in git. To install it run

``` shell,skip()
git config --global alias.fci '-c "core.editor=fast-conventional editor" commit'
```

To trigger it when you commit run

``` shell,skip()
git fci
```

## Further Docs

* [Shell completion](./docs/completion.md)
* [Configuration](./docs/configuration.md)
* [Editor](./docs/editor.md)
