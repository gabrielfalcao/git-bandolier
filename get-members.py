import os
import re
import io
import sys
import ipdb

from inspect import getmembers
from pathlib import Path
from typing import Any, Dict, List, Optional, Self, T, Tuple, TypeVar, Union

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


def get_module_members_only(mod: ModuleType) -> Dict[str, Any]:
    if not isinstance(mod, ModuleType):
        ty = type(mod)
        raise TypeError(
            f"argument `mod' expected {ModuleType} got {ty} {repr(mod)} instead"
        )

    module = mod
    got_members = getmembers(mod)
    all_members = list(got_members)
    members_dunder_module = [m for m in all_members if getattr(m, '__module__', None) == mod]
    ipdb.set_trace()
    result = [m for m in all_members if getattr(m, '__module__', None) == mod]


get_module_members_only(operator)
