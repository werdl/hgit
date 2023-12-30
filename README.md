# HGit
## Git, simpler
- HGit is a wrapper for Git that can be used as a drop in replacement. It has many useful tricks.
## Commands
- By default, your arguments are sent straight to the `git` executable (which must be installed on your system)
- There are also of course some HGit only commands
## `get [provider] repo`
- clones repo from provider
- supports github (`--github | -g`) and gitlab (`--gitlab | -l`)
### Examples
`hgit get werdl/hgit` - default pulls from github

`hgit get -l inkscape/inkscape` - from gitlab

`hgit get -g -l --github spartanproj/os` - last flag takes priority

## `go [commit message]`
- performs these operations, after concatenating all future arguments to one for `commit -m`
```bash
add .
commit -m <concat_message>
push
```
### Examples
`hgit go my first commit` - creates and pushes commit by name of `my first commit`

## `template [options] <template_addr>`
- creates a new repo, after cloning template_addr (does that in the same way `get` does, with provider flags)
- options must include `--name=<your_name>`
### Examples
`hgit start --name=cc werdl/c_project`

## `info`
- provides a much shorter output of all commits to main

## `data`
- shows the current status of the repository

## `update`
- grabs from remote

## `web`
- opens in web browser

## `activity`
- shows recent activity on all branches
