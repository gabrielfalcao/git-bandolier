#!/usr/bin/env python3

import sys
import click
import re
import shutil
import os
import json
import math
import subprocess
import urllib
import urllib.parse
import dataclasses

from pprint import pformat
from decimal import Decimal
from datetime import datetime, timedelta, UTC
from dataclasses import dataclass, field
from typing import List, Dict, Tuple, Union, Optional, Self
from pathlib import Path
from subprocess import Popen
import json
from pprint import pformat, pprint
from pathlib import Path
from itertools import chain
from collections import OrderedDict

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
