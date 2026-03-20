#!/usr/bin/env python3

import dataclasses
import json
import math
import os
import re
import shutil
import subprocess
import sys
import urllib
import urllib.parse
from collections import OrderedDict
from dataclasses import dataclass, field
from datetime import UTC, datetime, timedelta
from decimal import Decimal
from itertools import chain
from pathlib import Path
from pprint import pformat, pprint
from subprocess import Popen
from typing import Dict, List, Optional, Self, Tuple, Union

import click


def unique_list(items):
    return list(OrderedDict([(m, hash(m)) for m in items]).keys())

def scope():
    from_commands=unique_list([
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
    ])
    to_first_commands=unique_list([
        "st",
        "json",
        "br",
        "remotes",
        "ignore",
    ])

    to_existing_commands=unique_list([
        "actual_file_dates_commit",
        "add_and_commit_unfolded",
        "arcane_magic",
        "autocommit_plumbing",
        "autocommit_untracked_unstaged",
        "branch_format_the_struggle_aint_virtual",
        "commit",
        "commit_unstaged_and_untracked",
        "common_ancestor_to_ref",
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
        "q_broken_i_e_wip",
        "show_config",
        "status_porcelain_gawk",
        "status_print_path_if",
        "untracked",
    ])

    to_commands=unique_list(chain(to_first_commands, to_existing_commands))

    return locals()


@click.command()
def main():
    """
    refactors code, Thor's way
    """
    varbag = scope()
    neat = json.dumps(varbag, indent=4)
    print(f"varbag:\n{neat}")


if __name__ == "__main__":
    main()
