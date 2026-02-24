#!/usr/bin/env bash

set -umeTE
set +f
set -o pipefail

unset IFS
export IFS=$'\n'

declare -a subcommands=(
    "env"
    "context"
    "switch"
    "path"
    "sh"
    "goto"
    "list"
    "init"
    "doctor"
    "find"
    "show"
    "today"
    "update"
    "delete"
    "edit"
    "server"
    "client"
    "refresh"
    "shell"
    "tool"
    "parse"
    "export"
    "import"
    "web"
    "stash"
    "save"
    "load"
    "write"
    "read"
)

declare -- name=""
declare -- title=""
declare -- target_dir=""
declare -- target_path=""
declare -- mod_rsx=""
declare -- subcommands_dir="src/cli/subcommands"

declare -- from_name="bootstrap"
declare -- from_title=$(heck-string --to=pascal "${from_name}")
declare -- from_dir="${subcommands_dir}/${from_name}"

declare -- to_name=""
declare -- to_title=""
declare -- to_dir=""

declare -p ${!from_*}

for to_name in ${subcommands[@]}; do
    to_title=$(heck-string --to=pascal "${to_name}")
    to_dir="${subcommands_dir}/${to_name}"
    if [ -e "${to_dir}" ]; then
        rm -rf "${to_dir}"
    fi

    cp -rf "${from_dir}" "${to_dir}"
    refactors "${from_name}" "${from_name}" -wp "${to_dir}"
    refactors "${from_title}" "${from_title}" -wp "${to_dir}"
    git add -f "${to_dir}"
    git diff HEAD -- "${to_dir}"
    git commit "${to_dir}" -m "add boilerplate subcommand ${to_name}"
done
