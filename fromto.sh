#!/usr/bin/env bash

set -umeTE
set +f
set -o pipefail
unset IFS
export IFS=$'\n'

declare -- script_name="$(basename "${BASH_SOURCE[0]}")"
declare -- script_path="$(2>/dev/random 1>/dev/random cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
declare -- this_script_path="${script_path}/${script_name}"

declare -a from_commands=(
    "absorb"
    "add"
    "bootstrap"
    "client"
    "context"
    "delete"
    "doctor"
    "edit"
    "enqueue"
    "env"
    "export"
    "find"
    "goto"
    "import"
    "init"
    "list"
    "load"
    "parse"
    "path"
    "read"
    "refresh"
    "remove"
    "save"
    "server"
    "shell"
    "show"
    "stash"
    "switch"
    "task"
    "thatday"
    "today"
    "tool"
    "update"
    "web"
    "write"
)
declare -a to_commands=(
    "st"
    "json"
    "br"
    "remotes"
    "ignore"

    "actual-file-dates-commit"
    "add-and-commit-unfolded"
    "arcane-magic"
    "autocommit-plumbing"
    "autocommit-untracked-unstaged"
    "autocommit-untracked-unstaged.el"
    "br"
    "branch-format-the-struggle-aint-virtual"
    "commit"
    "commit-unstaged-and-untracked"
    "common-ancestor-to-ref"
    "diff-head"
    "dir"
    "filenames-diff-list"
    "files-changed-since"
    "files-changed-since.emacs-auto-save-2025-10-13-190841-UTC+0000"
    "g"
    "ignore"
    "import-new-files-from-remote-reference"
    "info"
    "linux-remote"
    "list-branches"
    "log-shash-date-subject"
    "modified"
    "offline"
    "onesum"
    "path"
    "q"
    "q-broken-i-e-wip"
    "qc"
    "remotes"
    "remotes.el"
    "show-config"
    "st"
    "status"
    "status-porcelain.gawk"
    "status-porcelain.gawk.el"
    "status-print-path-if"
    "untracked"

)

on_exit() {
    set +x
}
on_ctrlc() {
    1>&2 echo -e "\x1b[1;38;2;253;67;83m\rAborted with Ctrl-C\x1b[0m"
    exit 130
}
trap on_exit exit
trap on_ctrlc hup
trap on_ctrlc int
trap on_ctrlc bus
trap on_ctrlc segv
trap on_ctrlc sys

declare -a argv=(${@})
declare -i argc=${#argv[@]}

declare -i code=0

declare -i current=0
declare -- arg=""
declare -- pos=""

declare -i index=0
declare -- arg=""
declare -- param=""

declare -i next_index=0
declare -- next_arg=""
declare -- next_param=""

declare -i skip_next=0

declare -i lineno=0
declare -i line_number=0
declare -- line=""

declare -- argument=""
declare -- field=""
declare -- key=""
declare -- name=""
declare -- path=""
declare -- value=""

# <GIT>
declare -- git_repo_path=""
if ! git_repo_path=$(2>${stderr} git rev-parse --show-toplevel); then
    code=$?
fi
# </GIT>

main() {
    echo -e "from_commands: ${#from_commands[@]}"
    echo -e "to_commands: ${#to_commands[@]}"
}
if [ "${0}" == "${BASH_SOURCE[0]}" ]; then
    main
else
    1>&2 echo -e "${BASH_SOURCE[0]} appears to being used as a library by ${0@Q}"
fi
