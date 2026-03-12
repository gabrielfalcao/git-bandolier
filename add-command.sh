#!/usr/bin/env bash

set -umeTE
set +f
set -o pipefail

unset IFS
export IFS=$'\n'

declare -a commands=(
    "current"
    "now"
    "what"
    "wat"
    "git"
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

    echo -e  "\x1b[1;38;5;154madding command ${to_name}\x1b[0m"


    if [ -e "${to_dir}" ]; then
        rm -rf "${to_dir}"
    fi

    # echo copy "${from_dir}" "${to_dir}"
    cp -rf "${from_dir}" "${to_dir}"
    for rsfile in $(grep -l -E -r "(${from_name}|${from_title})" "${to_dir}"); do
        sed -E "s/${from_name}/${to_name}/g" -i "${rsfile}"
        sed -E "s/${from_title}/${to_title}/g" -i "${rsfile}"
    done
    cargo check

    # (replace-regexp
    #        regexp:   \(?:sed\(?:\s-*\(.*\)\s-*\)\)\(["']\)\(.*g\)\(\2\)
    #     to-string:    \,(regex!)
    #   )
    echo "
pub use commands::${to_name}::{
    ${to_title}Command, ${to_title}SharedOpt, ${to_title}DirOpt, ${to_title}FileOpt,
    ${to_title}Opt,
};

" >> ./src/cli/mod.rs
    cargo check

    echo "

pub mod ${to_name};
pub use ${to_name}::{
    ${to_title}Command, ${to_title}SharedOpt, ${to_title}DirOpt, ${to_title}FileOpt,
    ${to_title}Opt,
};


" >> ./src/cli/commands/mod.rs
    cargo check

# use workbench::cli::commands::{
#     BootstrapOpt, ClientOpt, ContextOpt, DeleteOpt, DoctorOpt, EditOpt,
#     EnqueueOpt, EnvOpt, ExportOpt, FindOpt, GotoOpt, ImportOpt, InitOpt,
#     ListOpt, LoadOpt, ParseOpt, PathOpt, ReadOpt, RefreshOpt, SaveOpt,
#     ServerOpt, ShellOpt, ShowOpt, StashOpt, SwitchOpt, TaskOpt, TodayOpt,
#     ToolOpt, UpdateOpt, WebOpt, WriteOpt,
# };
    #
    # stable \(?:sed\(?:\s-*\(.*\)\s-*\)\)\(["']\)\(.*g\)\(\2\) → \,(regex!)
    #
    sed -E "s,^(\s*)[/][/](\s*)((Command::)?${to_title}),\1\2\3,g" -i src/main.rs
    cargo check

    sed -E "s@${from_title}(${from_title}Opt),@${from_title}(${from_title}Opt),${from_title}(${from_titleOpt}),\n${to_title}(${to_titleOpt}),@g" -i src/main.rs
    cargo check

    sed -E "s@(Command::${from_title}(op) => op.dispatch()?)@Command::${from_title}(op) => op.dispatch()?,\n    Command::${to_title}(op) => op.dispatch()?,\n@g" -i src/main.rs
    cargo check
    sed -z -E 's@((use\s-+workbench::cli::commands::[{][[:space:]\n]+[^}]+Opt,)([[:space:]\n]*))[}][[:space:]\n]*,[[:space:]\n]*[;]@\1, ${to_title}Opt, \n[}];@g' -i src/main.rs
    cargo check

    if ! cargo run -- "${to_name}" --help; then
        echo -e  "\x1b[1;38;5;196mFAILED\x1b[0m"
        break
    fi

    git add -f "${to_dir}"
    git commit "${to_dir}" -m "add boilerplate subcommand ${to_name}"
done
