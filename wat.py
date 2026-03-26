# coding: utf-8
from shutil import *
from math import copysign, ceil, floor
from operator import *
from decimal import Decimal
from subprocess import Popen, STDOUT, PIPE, DEVNULL
from subprocess import *
from dataclasses import dataclass, field, fields
from typing import Any, Dict, List, Optional, Self, T, Tuple, TypeVar, Union
from collections.abc import Iterable, Iterator
import tomllib, math, argparse, asyncio, ast, abc, traceback, string, termios, subprocess, json, math, humanfriendly, operator, shutil, random, uiclasses, urllib, urllib.parse, decimal, urllib.request, dataclasses, datetime, click, codecs, builtins, sys, os, re, io
import collections, glob, fnmatch, IPython, pathlib, inspect, functools, itertools, io, _io, os, time, string, struct, sys, re, builtins, requests, socket
from pathlib import Path
from urllib.parse import *
from inspect import getmembers
from functools import *
from itertools import *
from pprint import pformat, pprint, pp
from collections import *
from fnmatch import fnmatchcase, fnmatch
from datetime import datetime, timedelta
from traceback import *
from types import *
import time

self = sys.modules[__name__]
self.__dict__.update(
    dict(
        list(
            chain(
                *[
                    [
                        (f"chars_{name}", sorted(value)),
                        (f"bytes_{name}", sorted(map(ord, value))),
                    ]
                    for name, value in getmembers(string)
                    if not (name.startswith("__") or name.endswith("__"))
                    and isinstance(value, str)
                ]
            )
        )
    )
)
chars_alphanumeric = sorted(
    chain(string.ascii_letters, string.digits, string.hexdigits, string.octdigits)
)
bytes_alphanumeric = sorted(map(ord, chars_alphanumeric))
chars_nonspace = sorted(
    chain(
        string.ascii_letters,
        string.digits,
        string.punctuation,
        string.hexdigits,
        string.octdigits,
    )
)
bytes_nonspace = sorted(map(ord, chars_nonspace))
# def get_actual_module_members(mod: ModuleType) -> List[Any]:    return [(name, member) for name, member in  inspect.getmembers(mod) if sys.modules.get(getattr(member, '__module__', '___NONE___'), None) == mod]
