# Validate Commits

You can check if a commit, or a range of commits are conventional

## Binary Usage

``` shell,script(name="help-validate",expected_exit_code=0)
fast-conventional validate --help
```

``` text,verify(script_name="help-validate",stream=stdout)
Validate a commit message is conventional

Usage: fast-conventional validate [OPTIONS] [REVISION_SELECTION]

Arguments:
  [REVISION_SELECTION]  An optional range to limit the linting

Options:
  -r, --repository <REPOSITORY_PATH>  Git repository to search in [env:
                                      FAST_CONVENTIONAL_GIT_REPOSITORY=] [default: .]
  -c, --config <CONFIG_PATH>          Configuration file [env: FAST_CONVENTIONAL_CONFIG=] [default:
                                      .fastconventional.yaml]
  -h, --help                          Print help
```

## Conventional Commits

Given we have created a git repository with all conventional commits

```shell,script(name="initialise-repository")
git init --template=/dev/null --quiet validate
git -C validate config user.name "Example Name"
git -C validate config user.email "name@example.com"
git -C validate config commit.gpgsign false
git -C validate commit --allow-empty -m "feat: Initial Release"
git -C validate commit --allow-empty -m "ci: Add pipeline"
```

and given we have this config

> `.fastconventional.yaml`

``` yaml,file(path=".fastconventional.yaml")
use_angular: true
types: [ci]
scopes: ["mergify", "just", "github"]
```

When we validate, we get a successful status

```shell,script(name="validate-fine",expected_exit_code=0)
fast-conventional validate -r validate
```

If we add a non-conventional commit

```shell,script(name="make-a-non-conventional-commit")
git -C validate commit --allow-empty -m "Non-coventional commit"
```

we get a failure

```shell,script(name="validate-non-conventional-commit",expected_exit_code=1)
fast-conventional validate -r validate
```

```text,verify(script_name="validate-non-conventional-commit", stream=stdout)
[✔] feat: Initial Release
[✔] ci: Add pipeline
```

```text,verify(script_name="validate-non-conventional-commit", stream=stderr)
[✘] Non-coventional commit
Error:   × Some commits failed validation

```

We can also restrict what we are validating, in this case we limit the range to a single commit

```shell,script(name="validate-commit-range",expected_exit_code=0)
fast-conventional validate  -r validate HEAD^^..HEAD^
```

```text,verify(script_name="validate-commit-range", stream=stdout)
[✔] ci: Add pipeline
```

It's also possible start from a specific commit and go back like with `git log`

```shell,script(name="validate-single-commit",expected_exit_code=0)
fast-conventional validate -r validate HEAD^
```

```text,verify(script_name="validate-single-commit", stream=stdout)
[✔] feat: Initial Release
[✔] ci: Add pipeline
```

We have seen a failure because of a non-conventional commit, we also might get a failure if we use a type that isn't in the configuration file

```shell,script(name="make-a-commit-with-unknown-type")
git -C validate commit --allow-empty -m "missing: Add a pipeline"
```


```shell,script(name="validate-missing-unknown-type",expected_exit_code=1)
fast-conventional validate -r validate HEAD^..HEAD
```

```text,verify(script_name="validate-missing-unknown-type", stream=stderr)
[✘] missing: Add a pipeline
Error: 
  × Some commits failed validation

```

You also validate the scopes

```shell,script(name="make-a-commit-with-unknown-type")
git -C validate commit --allow-empty -m "fix(invalid): Correct the automerge settings"
```


```shell,script(name="validate-missing-unknown-type",expected_exit_code=1)
fast-conventional validate -r validate HEAD^..HEAD
```

```text,verify(script_name="validate-missing-unknown-type", stream=stderr)
[✘] fix(invalid): Correct the automerge settings
Error: 
  × Some commits failed validation

```

These are optional unless you set `require_scope` in the config, in other words a missing scope won't fail the validation.

> `.fastconventional.yaml`
``` yaml,file(path=".fastconventional.yaml")
use_angular: true
require_scope: true
types: [ci]
scopes: ["mergify", "just", "github"]
```

```shell,script(name="make-a-commit-with-unknown-type")
git -C validate commit --allow-empty -m "fix: Correct the automerge settings"
```


```shell,script(name="validate-missing-unknown-type",expected_exit_code=1)
fast-conventional validate -r validate HEAD^..HEAD
```

```text,verify(script_name="validate-missing-unknown-type", stream=stderr)
[✘] fix: Correct the automerge settings
Error: 
  × Some commits failed validation

```
