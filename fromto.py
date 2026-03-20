#!/usr/bin/env python3
import os
import sys
import shutil
import json
import io
import click
import ipdb
import subprocess
from subprocess import CompletedProcess
from itertools import chain, zip_longest
from functools import reduce

def unique_list(items):
    return list(dict([(m, hash(m)) for m in items]).keys())


def scope():
    from_commands = unique_list(
        [
            "absorb",
            "add",
            "bootstrap",
            "client",
            "context",
            "delete",
            "doctor",
            "edit",
            "enqueue",
            "env",
            "export",
            "find",
            "goto",
            "import",
            "init",
            "list",
            "load",
            "parse",
            "path",
            "read",
            "refresh",
            "remove",
            "save",
            "server",
            "shell",
            "show",
            "stash",
            "switch",
            "task",
            "thatday",
            "today",
            "tool",
            "update",
            "web",
            "write",
        ]
    )
    to_first_commands = unique_list(
        [
            "st",
            "json",
            "br",
            "remotes",
            "ignore",
            "establish",
        ]
    )

    to_existing_commands = unique_list(
        [
            "actual_file_dates_commit",
            "add_and_commit_unfolded",
            "arcane_magic",
            "autocommit_plumbing",
            "autocommit_untracked_unstaged",
            "branch_format_the_struggle_aint_virtual",
            "commit",
            "commit_unstaged_and_untracked",
            "common_ancestor",
            "filenames_diff_list",
            "files_changed_since",
            "init_g",
            "quickcommit",
            "import_new_files_from_remote_reference",
            "info",
            "linux_remote",
            "list_branches",
            "log_shash_date_subject",
            "offline",
            "onesum",
            "path",
            "show_config",
            "status_porcelain_gawk",
            "status_print_path_if",
            "untracked",
        ]
    )

    to_commands = unique_list(chain(to_first_commands, to_existing_commands))

    return from_commands, to_commands, to_existing_commands, to_first_commands

HECK_FROM_SNAKE_TO_VARIANTS= ['kebab', 'camel', 'pascal', 'shouty-kebab', 'shouty-snake', 'train', 'title', 'snake']

def heck_string(value: str, to: str = 'pascal', **extra_popen_kwargs) -> Dict[str, Any]:
    prog = shutil.which("heck-string")
    inputs = value.split()
    popen_prog_args = [
            prog,
            f"--to={to}",
    ]
    popen_prog_args.extend(inputs)

    popen_initial_kwargs = dict(
        # close_fds=True,
        # stdout=subprocess.PIPE,
        # stderr=subprocess.PIPE,
        # stdin=subprocess.DEVNULL,
        text=True,
    )
    popen_core_kwargs = dict(
        args=popen_prog_args,
    )
    popen_base_kwargs = dict(
        **popen_core_kwargs,
        **popen_initial_kwargs,
    )
    popen_kwargs = dict(
        **popen_base_kwargs,
        **extra_popen_kwargs,
    )
    popen_kwargs.update(popen_core_kwargs)
    popen_kwargs.update(dict(capture_output=True))
    popen_kwargs.pop('stderr', None)
    popen_kwargs.pop('stdout', None)
    proc = subprocess.run(**popen_kwargs)
    outputs = proc.stdout.strip()
    return dict(to=to, inputs=inputs, outputs=outputs)


def heck_variants(snake_name) -> List[Dict[str, Any]]:
    call_kw_list = [dict(to=to, value=snake_name) for to in HECK_FROM_SNAKE_TO_VARIANTS]
    return [heck_string(**kw) for kw in call_kw_list]

def command_variants(snake_name) -> List[Dict[str, Any]]:
    command_module_name = heck_string(snake_name, to='snake')
    command_pascal_name = heck_string(snake_name, to='pascal')
    command_opt_name = f"{command_pascal_name}Opt"
    command_command_name = f"{command_pascal_name}Command"
    command_subcommand_name_file = f"{command_pascal_name}FileOpt"
    command_subcommand_name_dir = f"{command_pascal_name}DirOpt"
    call_kw_list = [dict(to=to, value=snake_name) for to in HECK_FROM_SNAKE_TO_VARIANTS]
    return [heck_string(**kw) for kw in call_kw_list]

def repurpose_command(from_snake, to_snake):
    from_variants = heck_variants(from_snake)
    to_variants = heck_variants(to_snake)
    ipdb.set_trace()
    return from_variants, to_variants

@click.command()
def main():
    """
    refactors code, Thor's way
    """
    from_commands, to_commands, _to_existing_commands, _to_first_commands = scope()
    for from_snake, to_snake in zip_longest(from_commands, to_commands):
        repurpose_command(from_snake, to_snake)
    # print(f'from_commands: {len(from_commands)}')
    # print(f'to_commands: {len(to_commands)}')


if __name__ == "__main__":
    main()
