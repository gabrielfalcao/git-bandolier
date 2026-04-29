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

declare -- new_command_name=$(heck-string --to="snake" <<< "${argv[0]}")
cp -rfv "${script_path}/switch/"  "${script_path}/${new_command_name}/"

git add  "${script_path}/${new_command_name}"
git commit  "${script_path}/${new_command_name}" -m "adds new command ${new_command_name@Q}"

for variant in ${hecks[@]}; do
    cur_var=$(heck-string --to="${variant}" "switch")
    dst_var=$(heck-string --to="${variant}" "remotes")

    if (cd "${script_path}/${new_command_name}" &&  refactors "${cur_var}" "${dst_var}" -wp .); then
        git add -f .
        git commit . -m "from ${cur_var} to ${dst_var}"
    fi
done
