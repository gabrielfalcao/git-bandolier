gc3() {
    #set -x
    . ~/opt/lib/ansi.sh
    set +ue
    export IFS=$'\n'
    repo=""
    local -a argv=($@)
    local -- argc=$((${#argv} + 0))
    local -a git_argv=()
    repo_url=""
    default_target_path="$HOME/projects/third_party"
    target_path="$default_target_path"
    local -- argv_array_items="$(echo -n "${argv[@]}" | tr '[:space:]' ',')"
    for idx in ${!argv[@]}; do
        nidx=$(($idx + 1))
        arg=${argv[$idx]}
        next_arg=${argv[$nidx]}
        if [ -n "${next_arg}" ] && 1>/dev/random 2>/dev/random printf "%d" "${next_arg}"; then
            next_arg_num=$(("${next_arg}"))
        fi
        case "$arg" in
            "--help" | "help")
                echo -e 'NAME
    gc3 - git-clones third-party

SYNOPSIS
    gc3 [-d|--depth] [-b|--backup] [-n|--namespace] <URL>

DESCRIPTION

    shallow clones git repository inside of ~/projects/third_party and
    automatically tries to build the project, run tests, code examples
    and/or open the documentation of the project in the target git
    repository.

    (currently supports rust, nodejs and golang projects and works best in rust projects)

    Options:

      -n, --namespace
          places the git repository inside of `$HOME/projects/third_party/<REPOSITORY OWNER>/<REPOSITORY NAME>\x27
          instead of the default ($HOME/projects/third_party/<REPOSITORY NAME>)

      -d, --depth <NUMBER>
          if this option is provided, the <NUMBER> value must be
          an unsigned, non-zero number which is then forwarded to the corresponding `--depth\x27 option in the
          underlying git command (.i.e.: `git clone --depth <GIT_REMOTE_URL>\x27)

          Default value: 1

      -b, --backup
          by default gc3 skips cloning a git repository if the target
          path in the filesystem already exists, however, if this
          option is specified then an existing target path is
          backed-up and a new copy of the target repo is cloned there.
'
                return 0
                ;;
            "-d" | "--depth")
                if [ -n "${next_arg_num}" ]; then
                    depth=$next_arg_num
                else
                    error_function "missing numeric value of --depth argument on position ${nidx} of [${argv_array_items}]"
                    return 1
                fi
                git_argv+=("--depth" "${depth}")
                ;;
            "-b" | "--backup")
                backup="true"
                ;;
            "-n" | "--namespace")
                if [ -z "$target_path" ] || [ "${target_path}" == "${default_target_path}" ] || [ -e "$target_path" ]; then
                    target_path="${default_target_path}/${owner_name}"
                else
                    error_function "target_path already set to: ${target_path}"
                    return 1
                fi
                ;;
            *)
                if 2>/dev/random 1>/dev/random grep '^\(https\?\|git\)\(://\|@\)\([^/]\+\)\(.*\)\([.]git\)\?$' <<<"${arg}"; then
                    if [ -z "$repo" ]; then
                        repo="$arg"
                        repo_url="$arg"
                    else
                        error_function "repo already set to: ${repo@Q}"
                        return 1
                    fi
                else
                    1>&2 bar_text_left 231 196 "${arg@Q} does not seem to be a git repo address"
                    return 1
                fi
                ;;
        esac
    done
    if [ -z "$repo" ]; then
        clipboard="$(pbpaste)"
        if 2>/dev/random 1>/dev/random grep '^\(https\?\|git\)\(://\|@\)\([^:/]\+\)\(.*\)\([.]git\)\?$' <<<"${clipboard}"; then
            repo="${clipboard}"
        fi
    fi
    if [ -z "$repo" ]; then
        bar_text_left 231 196 "ERROR: missing git repo url"
        return 1
    fi
    repo="$(sed 's/git@\([^:]\+\):/https:\/\/\1\//g' <<<"${repo}")"
    repo="$(sed 's/\(^"\+\|"\+$\)//g' <<<"${repo}")"
    repo="$(sed "s/\(^'\+\|'\+$\)//g" <<<"${repo}")"
    gits="$(sed 's/^\(\(https\?:\/\/\|git@\|git:\/\/\)[^[:space:]]\+\([.]git\)\?\)$/gitrepo/g' <<<"$repo")"

    if 2>/dev/random 1>/dev/random grep "https://crates.io/crates/" <<<"${repo}"; then
        crate_name=$(echo "$repo" | sed 's/^\s*https:\/\/crates[.]io\/crates\/\+\([^\/?]\+\).*/\1/g')
        bar_text_left 82 16 "crate name: ${crate_name}"
        if ! repo=$(crates_io_get_git_repo_url "${crate_name}"); then
            bar_text_left 231 196 "ERROR: failed to fetch json from ${crates_api_url}"
            return 1
        fi
    elif 2>/dev/random 1>/dev/random grep "https://docs[.]rs/[a-z_-]\+" <<<"${repo}"; then
        crate_name=$(echo "$repo" | sed 's/^\s*https:\/\/docs[.]rs\/\+\([^\/?]\+\).*/\1/g')
        bar_text_left 82 16 "crate name: ${crate_name}"
        if ! repo=$(crates_io_get_git_repo_url "${crate_name}"); then
            bar_text_left 231 196 "ERROR: failed to fetch json from ${crates_api_url}"
            return 1
        fi
    fi
    if 1>/dev/random 2>/dev/random grep '[a-z0-9_+-]\+@' <<<"${repo}"; then
        repo="$(echo -n "$repo" | tr -d '[:space:]' | sed 's/\/\?\([.]git\)\?$//g' | sed 's/\(https:\/\/[^?]\+\)[?].*$/\1/g' | sed 's/git@[^:]\+://g')"
    else
        repo="$(echo -n "$repo" | tr -d '[:space:]' | sed 's/\/\?\([.]git\)\?$//g' | sed 's/\(https:\/\/[^?]\+\)[?].*$/\1/g')"
    fi

    owner_name="$(sed 's/^\(https:\/\/\|git:\/\/\|git@\)[^\/:]\+:\?\/\([^\/]\+\)\/\([^\/]\+\)/\2/g' <<<${repo})"
    repo_name="$(sed 's/^\(https:\/\/\|git:\/\/\|git@\)[^\/:]\+:\?\/\([^\/]\+\)\/\([^\/]\+\)/\3/g' <<<${repo})"
    # echo "owner_name=${owner_name@Q}";
    # echo "repo_name=${repo_name@Q}";

    if [ ${#git_argv} -eq 0 ]; then
        git_argv+=("--depth" "1")
    fi

    if [ -z "${repo_name}" ]; then
        bar_text_left 231 196 "ERROR: could not determine repo name from url: ${repo@Q}"
        return 1
    fi
    if [ -z "${owner_name}" ]; then
        bar_text_left 231 196 "ERROR: could not determine owner name from url: ${repo@Q}"
        return 1
    fi

    if [ "$gits" != "gitrepo" ]; then
        bar_text_left 231 196 "ERROR: invalid git repo url: ${repo_url@Q}"
        return 1
    fi

    tput clear
    mkdir -p "$target_path"
    target_repo_path="${target_path}/$repo_name"
    if [ -e "$target_repo_path" ]; then
        if [ "${backup}" == "true" ]; then
            mv "$target_repo_path" "$target_path/${repo_name}-$(t16g)"
        else
            bar_text_center 101 16 "into existing ${target_repo_display}"
        fi
    else
        bar_text_center 121 16 "cloning ${repo_name} to \x1b[1;38;5;233m${target_repo_path}"
        git clone "${repo}.git" ${git_argv[@]} "$target_repo_path"
    fi
    cd "$target_repo_path"
    ls -latrh
    if [ -f ".gitmodules" ]; then
        git submodule init
        git submodule update
    fi
    if [ -f "Cargo.toml" ]; then
        if [ -f "rust-toolchain.toml" ]; then
            sed 's/channel\s*=\s*"[^"]\+"/channel = "nightly-2025-09-09"/g' -i rust-toolchain.toml
        fi
        if [ -d "examples" ]; then
            varname="${repo_name//-/_}_path"
            varname="${varname@L}"
            cat >cargo-run-examples.sh <<EOF
#!/usr/bin/env bash

. \$HOME/opt/lib/ansi.sh

ctrlc() {
    echo -e "\r"
    ansi_up
    bar_text_left 196 231 "Cancelled with Control-C"
    reset
    2>/dev/null 1>/dev/null stty sane
    exit 1
}
trap ctrlc int

${varname}="${target_repo_path}"

bar 235
failed=""
for example in \$(2>/dev/null wc -l \${${varname}}/examples/*.rs | sort -n | grep -v 'total\$' | lastcol | xargs-each basename); do
    example=\${example/%.rs/}
    bar_text_left 222 233 "cargo run --example \${example}"
    if ! cargo run --example \${example}; then
        if [ -z "${failed}" ] || [ "${failed}" != "false" ]; then
            rm -rf target
            break
        else
            failed="true"
        fi
    else
        bar_text_left 222 233 "\$(ansi_spaced 233 222 "FINISHED")\$(ansi_spaced 222 233 "cargo run --example \${example}")"
        failed="false"
    fi
done

EOF

            chmod +x cargo-run-examples.sh
            ./cargo-run-examples.sh
        elif [ -f "src/main.rs" ] || [ $(ack '[[]bin[]]' Cargo.toml | wc -l) -gt 0 ]; then
            bar_text_left 222 233 "cargo run # (${target_repo_path})"
            if ! cargo run; then
                rm -rf target
            else
                bar_text_left 222 233 "$(ansi_spaced 233 222 "FINISHED")$(ansi_spaced 222 233 "cargo run")"
            fi
        elif [ $(ack '[[]workspace[]]' Cargo.toml | wc -l) -gt 0 ]; then
            bar 235
            failed=""
            for bin_name in $(find . -type f -name Cargo.toml -exec bash -c 'test -f $(dirname {})/src/main.rs && grep "^[[:space:]]*name\s*=\s*" {} | sed "s/^name\s*=\s*\"\([^\"]\+\)\".*/\1/g"' \;); do
                bar_text_left 222 233 "cargo run --bin ${bin_name} # (${target_repo_path})"
                if ! cargo run --bin ${bin_name}; then
                    if [ -z "${failed}" ] || [ "${failed}" != "false" ]; then
                        rm -rf target
                        break
                    else
                        failed="true"
                    fi
                else
                    bar_text_left 222 233 "$(ansi_spaced 233 222 "FINISHED")$(ansi_spaced 222 233 "cargo run --bin ${bin_name}")"
                    failed="false"
                fi
            done
        else
            bar_text_center 154 235 "running cargo cbt"
            cargo cbt
        fi
        if web_browser_is_open; then
            cargo doc --open
        else
            cargo doc
        fi
    fi
    if [ -f "package.json" ]; then
        bar_text_left 154 235 'package.json detected'

        node_run_command=(npm run)
        if [ -f "package-lock.json" ]; then
            npm i
            node_run_command=(npm run)
        elif [ -f "yarn-lock.json" ]; then
            yarn
            node_run_command=(yarn)
        elif [ -f "pnpm-lock.json" ]; then
            pnpm i
            node_run_command=(pnpm)
        fi
        local -a script_names=($(jq '.scripts | keys[]' package.json | ansistrip | tr -d '"'))
        local -a autorun_build=()
        local -a autorun_script_names=()

        if [ ${#script_names[@]} -gt 0 ]; then
            echo -e "\x1b[1;38;5;159mpackage.json scripts\x1b[0m"
            fgcolor=159
            for script_name in ${script_names[@]}; do
                case "${script_name}" in
                    "build" | "compile")
                        autorun_build+=("${script_name}")
                        fgcolor=220
                        ;;
                    "dev" | "start" | "start:dev" | "dev:start")
                        autorun_script_names+=("${script_name}")
                        fgcolor=154
                        ;;
                    *)
                        fgcolor=159
                        ;;
                esac
                echo -e "    \x1b[1;38;5;${fgcolor}m${node_run_command[@]} ${script_name}\x1b[0m"
            done
            for script_name in ${autorun_build[@]}; do
                fgcolor=220
                echo -e "\x1b[1;38;5;${fgcolor}m${node_run_command[@]} ${script_name}\x1b[0m"
                ${node_run_command[@]} ${script_name}
            done
            for script_name in ${autorun_script_names[@]}; do
                fgcolor=154
                echo -e "\x1b[1;38;5;${fgcolor}m${node_run_command[@]} ${script_name}\x1b[0m"
                ${node_run_command[@]} ${script_name}
            done
        fi
    fi
    if [ -f "go.mod" ]; then
        go_version=$(2>/dev/random go version)
        go_version=${go_version/#go version go/}
        go_version=${go_version/% */}
        if [ -n "${go_version}" ]; then
            go build .
        fi
    fi
}
