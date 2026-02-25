#!/usr/bin/env bash

set -umeTE
set +f
set -o pipefail

unset IFS
export IFS=$'\n'

declare -a commands=(
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
declare -- commands_dir="src/cli/commands"

declare -- from_name="bootstrap"
declare -- from_title=$(heck-string --to=pascal "${from_name}")
declare -- from_dir="${commands_dir}/${from_name}"

declare -- to_name=""
declare -- to_title=""
declare -- to_dir=""
declare -- rsfile=""

declare -p ${!from_*}

echo -en "\x1b[2J\x1b[3J\x1b[H"
for to_name in ${commands[@]}; do
    to_title=$(heck-string --to=pascal "${to_name}")
    to_dir="${commands_dir}/${to_name}"
    if [ -e "${to_dir}" ]; then
        rm -rf "${to_dir}"
    fi

    # echo copy "${from_dir}" "${to_dir}"
    cp -rf "${from_dir}" "${to_dir}"
    for rsfile in $(grep -l -E -r "(${from_name}|${from_title})" "${to_dir}"); do
        sed -E "s/${from_name}/${to_name}/g" -i "${rsfile}"
        sed -E "s/${from_title}/${to_title}/g" -i "${rsfile}"
    done

    echo "
pub use commands::${to_name}::{
    ${to_title}Command, ${to_title}SharedOpt, ${to_title}DirOpt, ${to_title}FileOpt,
    ${to_title}Opt,
};

" >> ./src/cli/mod.rs

    echo "

pub mod ${to_name};
pub use ${to_name}::{
    ${to_title}Command, ${to_title}SharedOpt, ${to_title}DirOpt, ${to_title}FileOpt,
    ${to_title}Opt,
};

" >> ./src/cli/commands/mod.rs

    sed -E "s,^(\s*)[/][/](\s*)((Command::)?${to_title}),\1\2\3,g" -i src/main.rs

    if cargo run; then
        git add -f "${to_dir}"
        git diff HEAD -- "${to_dir}"
        continue
    fi
    break
    # git commit "${to_dir}" -m "add boilerplate subcommand ${to_name}"
done
