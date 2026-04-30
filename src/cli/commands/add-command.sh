#!/usr/bin/env bash

set -umeTE
set +f
set -o pipefail
unset IFS
export IFS=$'\n'

declare -- script_name="$(basename "${BASH_SOURCE[0]}")"
declare -- script_path="$(2>/dev/random 1>/dev/random cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
declare -- git_repo_path=""
declare -a argv=(${@})
declare -i argc=${#argv[@]}

# <GIT>
if git_repo_path=$(2>${stderr} git rev-parse --show-toplevel); then
    code=0
else
    code=$?
fi
if [ ${code} -ne 0 ]; then
    1>&2 echo -e "\x1b[0m\x1b[1;48;2;253;67;83m[${script_name} warning]\x1b[7m $(pwd) is not under git version control\x1b[0m"
fi
# </GIT>
export IFS=$'\n'

declare -a hecks=(snake pascal shouty_snake)

declare -- new_command_input_path="${script_path}/${new_command_name}/"
declare -- new_command_abs_path=$(path canon "${new_command_input_path}")
declare -- new_command_path_relative_to_git_repo=${new_command_abs_path#${git_repo_path}/}

declare -- new_command_name=$(heck-string --to=snake "$(slugify-string "${argv[1]}")")
# declare -- new_command_name=$(heck-string --to="snake" <<< "${argv[1]}")
cp -rfv "${script_path}/switch/"  "${script_path}/${new_command_name}/"

declare -- from_input="switch"
declare -- to_input="${new_command_name}"
declare -- from=""
declare -- to=""
declare -- new_command_path_relative_to_git_repo=$(path canon "${}")
declare -i code=0
for variant in ${hecks[@]}; do
    from=$(heck-string --to="${variant}" "switch")
    to=$(heck-string --to="${variant}" "${new_command_name}")

    cd "${git_repo_path}"

    code=0
    if refactors "${cur_var}" "${dst_var}" -wp "${new_command_abs_path}"; then
        if (cd "${git_repo_path}" && git add -f "${script_path}/${new_command_name}" && git commit "${script_path}/${new_command_name}" -m "from ${cur_var} to ${dst_var}"); then
            code=0
        else
            code=$?
        fi
    fi
done
