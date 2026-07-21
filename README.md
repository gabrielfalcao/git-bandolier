# Git Bandolier

[![./logo.png](./logo.png)](https://crates.io/crates/git-bandolier)

Git Bandolier is a small set of useful git subcommands which, when
applicable and reasonable, displays colored information which can be
very helpful to developers like myself who suffer from ADHD.


## Installation


Git Bandolier installs a set of commands prefixed with `git-` which
git automatically turns into `git` subcommands. (.e.g.: `git-br` becomes `git br`)

```shell
cargo install git-bandolier
```


## Commands


### `git path`

prints the repository path


```shell
git path
```


### `git br`

lists, creates and deletes local branches

#### Listing Branches with automatic coloring

```shell
git br
```

or
```shell
git list-branches
```


#### Creating a local branch

```shell
git br new-branch
```

#### Deleting a local branch

```shell
git br -D new-branch
```


### `git remotes`

lists remotes


```shell
git remotes
```
