#!/usr/bin/env bash

set -umeTE
set +f
set -o pipefail
unset IFS
export IFS=$'\n'
### 0=`declare -a to_existing_commands`
### 1=`declare -a to_existing_commands`
### 2=`to_existing_commands`
### 3=`to_existing`
### 4=`_existing`
### 5=`_commands`

declare -- script_name="$(basename "${BASH_SOURCE[0]}")"
declare -- script_path="$(2>/dev/random 1>/dev/random cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
declare -- this_script_path="${script_path}/${script_name}"
declare -- stderr="$(mktemp)"
# ZGVjbGFyZSAtQSBmcm9tX3RvX2NvbW1hbmRzX21hcD0oKQ==
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
declare -a to_first_commands=(
    "st"
    "json"
    "br"
    "remotes"
    "ignore"
)

declare -a to_existing_commands=(
    "actual_file_dates_commit"
    "add_and_commit_unfolded"
    "arcane_magic"
    "autocommit_plumbing"
    "autocommit_untracked_unstaged"
    "branch_format_the_struggle_aint_virtual"
    "commit"
    "commit_unstaged_and_untracked"
    "common_ancestor_to_ref"
    "filenames_diff_list"
    "files_changed_since"
    "init_g"
    "quickcommit"
    "import_new_files_from_remote_reference"
    "info"
    "linux_remote"
    "list_branches"
    "log_shash_date_subject"
    "offline"
    "onesum"
    "path"
    "q_broken_i_e_wip"
    "show_config"
    "status_porcelain_gawk"
    "status_print_path_if"
    "untracked"
)
declare -a to_commands=(
    ${to_first_commands[@]}
    ${to_existing_commands[@]}
)
declare -gi from_commands_count=${#from_commands[@]}
declare -gi to_commands_count=${#to_commands[@]}
declare -gi from_commands_index=0
declare -gi to_commands_index=0
declare -gi from_commands_current=0
declare -gi to_commands_current=0
declare -g from_commands_value=""
declare -g to_commands_value=""
declare -g from_commands_pos=""
declare -g to_commands_pos=""

on_exit() {
    bash -c "rm -f ${stderr@Q} &
disown -a"
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
if git_repo_path=$(2>${stderr} git rev-parse --show-toplevel); then
    code=0
else
    code=$?
fi
# </GIT>

cls() {

    1>&2 echo -en "\x1b[2J\x1b[3J\x1b[H"
    1>&2 echo -e '\n'

}
main() {
    # declare -A from_to_first_commands_map=()

    1>&2 echo -en "\x1b[2J\x1b[3J\x1b[H"
    echo -e "${#from_commands[@]} from_commands: ${from_commands[@]@Q}\n"
    echo -e "${#to_first_commands[@]} to_first_commands: ${to_first_commands[@]@Q}\n"
    echo -e "${#to_existing_commands[@]} to_existing_commands: ${to_existing_commands[@]@Q}\n"
    echo -e "${#to_commands[@]} to_commands: ${to_commands[@]@Q}\n"

    from_commands_count=${#from_commands[@]}
    to_commands_count=${#to_commands[@]}
    local -- varname=""
    local -- line=""
    local -a varnames=()
    cls
    echo
    for from_commands_index in ${!from_commands[@]}; do
        from_commands_index=0
        from_commands_current=0
        from_commands_value=""
        from_commands_pos=""

        from_commands_current=$((from_commands_index + 1))
        from_command_value="${from_commands[${from_commands_index}]}"
        from_commands_pos="$(printf '%*s of %s' ${#from_commands_count} ${from_commands_current} ${from_commands_count})"

        for to_commands_index in ${!to_commands[@]}; do
            to_commands_index=0
            to_commands_current=0
            to_commands_value=""
            to_commands_pos=""

            to_commands_current=$((to_commands_index + 1))
            to_command_value="${to_commands[${to_commands_index}]}"
            to_commands_pos="$(printf '%*s of %s' ${#to_commands_count} ${to_commands_current} ${to_commands_count})"
            1>&2 echo -en '\x1b[1;48;2;85;87;83m\x1b[1;38;2;138;226;52m'
            varnames=($(echo "${!from_*} ${!to_*}"))
            1>&2 echo -en "varnames:\x1b[0m\n"
            for varname in ${varnames[@]}; do
                if [[ -v refvar ]]; then
                    unset -n refvar
                fi
                local -I -n refvar="${varname}"
                1>&2 echo -en '\x1b[1;38;2;85;87;83m\x1b[1;48;2;138;226;52m'
                1>&2 echo -en "${refvar[@]@Q}"
                1>&2 echo -e '\x1b[0\n'
            done
            1>&2 echo -e '\x1b[0\n'
        done
    done

    # 0=`local -- pos=`
    # 1=`--`
    # 2=`-`
    # 3=`pos`
    exit 11

    local -- from_cmd=""
    local -- from_name=""
    local -- from_path=""
    local -i from_index=0
    local -i from_current=0
    local -- from_arg=""
    local -- from_pos=""

    local -- to_cmd=""
    local -- to_name=""
    local -- to_path=""
    local -i to_index=0
    local -i to_current=0
    local -- to_arg=""
    local -- to_pos=""

    for from_index in ${!from_commands[@]}; do
        from_current=$(($from_index + 1))
        from_arg="${argv[$from_index]}"
        from_pos="$(printf '%*s of %s' ${#argc} ${from_current} ${argc})"

    done

}
if [ "${0}" == "${BASH_SOURCE[0]}" ]; then
    main
else
    1>&2 echo -e "${BASH_SOURCE[0]} appears to being used as a library by ${0@Q}"
fi
