import os
import re
import io
import sys
import json
import ipdb

import dataclasses
from dataclasses import dataclass, field
from inspect import getmembers
from pathlib import Path
from typing import Any, Dict, List, Optional, Self, T, Tuple, TypeVar, Union
from types import ModuleType

import shutil
import operator
import subprocess
import urllib
import urllib.parse
import functools
import itertools
import collections
import traceback
import types

modules = [
    shutil,
    operator,
    subprocess,
    urllib,
    urllib.parse,
    functools,
    itertools,
    collections,
    traceback,
    types,
]

@dataclass
class ModuleMember:
    obj: Any
    module_name: str
    module_path: str

    module_name: str = field(init=False)
    module_path: Optional[str] = field(init=False)

    def __post_init__(self, **kw_init):
        uri = urllib.parse.urlparse(self.cwd)
        working_dir = Path(uri.path)
        self.working_dir = working_dir

        tty = Path(self.tty_name)
        tty_stat = tty.stat()
        self.accessed_unix = int(tty_stat.st_atime)
        self.changed_unix = int(tty_stat.st_ctime)
        wz = wezterm(pane_id=self.pane_id)
        output = wz.get_text()
        self.output = output



def get_module_members_only(mod: ModuleType) -> Dict[str, Any]:
    if not isinstance(mod, ModuleType):
        ty = type(mod)
        raise TypeError(
            f"argument `mod' expected {ModuleType} got {ty} {repr(mod)} instead"
        )

    module = mod
    got_members = getmembers(mod)
    all_members = list(got_members)
    members_dunder_module = [(key, value) for key, value in all_members if getattr(value, '__module__', None)]
    ipdb.set_trace()
    result = [(key, value) for key, value in all_members if getattr(value, '__module__', None)]



get_module_members_only(operator)
