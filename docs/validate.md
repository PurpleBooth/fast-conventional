# Validate Commits

You can check if a commit, or a range of commits are conventional

## Binary Usage

``` shell,script(name="help-validate",expected_exit_code=0)
fast-conventional validate --help
```

``` text,verify(script_name="help-validate",stream=stdout)
fast-conventional-validate 
Validate a commit message is conventional

USAGE:
    fast-conventional validate [OPTIONS] [REVISION_SELECTION]

ARGS:
    <REVISION_SELECTION>    An optional range to limit the linting

OPTIONS:
    -h, --help
            Print help information

    -r, --repository <REPOSITORY_PATH>
            Git repository to search in [env: FAST_CONVENTIONAL_GIT_REPOSITORY=] [default: .]
```

## Conventional Commits

Given we have created a git repository with all conventional commits

```shell,script(name="initialise-repository")
git init --template=/dev/null --quiet .
git config user.name "Example Name"
git config user.email "name@example.com"
git config commit.gpgsign false
git commit --allow-empty -m "feat: Initial Release"
git commit --allow-empty -m "ci: Add pipeline"
```

When we validate, we get a successful status

```shell,script(name="validate-fine",expected_exit_code=0)
fast-conventional validate
```

If we add a non-conventional commit

```shell,script(name="make-a-non-conventional-commit")
git commit --allow-empty -m "Non-coventional commit"
```

we get a failure

```shell,script(name="validate-non-conventional-commit",expected_exit_code=1)
fast-conventional validate
```

```text,verify(script_name="validate-non-conventional-commit", stream=stdout)
[✔] feat: Initial Release
[✔] ci: Add pipeline
```

```text,verify(script_name="validate-non-conventional-commit", stream=stderr)
[✘] Non-coventional commit
Error: 
  × Some commits failed validation

```

We can also restrict what we are validating, in this case we limit the range to a single commit

```shell,script(name="validate-commit-range",expected_exit_code=0)
fast-conventional validate HEAD^^..HEAD^
```

```text,verify(script_name="validate-commit-range", stream=stdout)
[✔] ci: Add pipeline
```

It's also possible start from a specific commit and go back like with `git log`

```shell,script(name="validate-single-commit",expected_exit_code=0)
fast-conventional validate HEAD^
```

```text,verify(script_name="validate-single-commit", stream=stdout)
[✔] feat: Initial Release
[✔] ci: Add pipeline
```
