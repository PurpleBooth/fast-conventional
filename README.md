![Fast
Conventional](https://raw.githubusercontent.com/PurpleBooth/fast-conventional/main/logo/logo-light.svg#gh-light-mode-only)
![Fast
Conventional](https://raw.githubusercontent.com/PurpleBooth/fast-conventional/main/logo/logo-dark.svg#gh-dark-mode-only)

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
page](https://codeberg.org/PurpleBooth/fast-conventional/releases/latest)
for binaries, and see the [packages
page](https://codeberg.org/PurpleBooth/fast-conventional/packages) for
RPM, Arch, Alpine, Debian and Docker repositories.

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

- [Shell completion](./docs/completion.md)
- [Configuration](./docs/configuration.md)
- [Editor](./docs/editor.md)
- [Validate](./docs/validate.md)
- [Cli usage](./docs/cli-usage.md)
