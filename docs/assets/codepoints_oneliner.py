# coding: utf-8
import inspect, functools, itertools, io, os, sys, re, builtins;from inspect import getmembers; from functools import *;from itertools import *;from pprint import pformat, pprint, pp
import collections, inspect, functools, itertools, io, os, sys, re, builtins;from inspect import getmembers; from functools import *;from itertools import *;from pprint import pformat, pprint, pp;from collections import *;
regexp_boundary_chars=list(OrderedDict([(c, c) for c in chain(list('([{|}])^$'), list('"'), list("'"))]).keys())
regexp_boundary_chars
import collections, inspect, functools, itertools, io, os, sys, re, builtins;from inspect import getmembers; from functools import *;from itertools import *;from pprint import pformat, pprint, pp;from collections import *;  \
regexp_boundary_chars=list(OrderedDict([(c, c) for c in chain(list('([{|}])^$'), list('"'), list("'"))]).keys()); \
regexp_boundary_char_map=list(dict([(c, ord(c)) for c in chain(list('([{|}])^$'), list('"'), list("'"))]).keys())
regexp_boundary_char_map
import collections, inspect, functools, itertools, io, os, sys, re, builtins;from inspect import getmembers; from functools import *;from itertools import *;from pprint import pformat, pprint, pp;from collections import *;  \
regexp_boundary_chars=list(OrderedDict([(c, c) for c in chain(list('([{|}])^$'), list('"'), list("'"))]).keys()); \
regexp_boundary_char_map=OrderedDict([(c, ord(c)) for c in chain(list('([{|}])^$'), list('"'), list("'"))])
regexp_boundary_char_map
regexp_boundary_char_map_labels=('char', 'codepoint')
# [[enumerate(regexp_boundary_char_map_labels)] for char, codepoint in regexp_boundary_char_map.items()]
[locals() for char, codepoint in regexp_boundary_char_map.items()]
[dict([(label, kv_charpoint[index]) for index, label in enumerate(regexp_boundary_char_map_labels)]) for kv_charpoint in regexp_boundary_char_map.items()]
[dict([(label, kv_charpoint[index]) for index, label in enumerate(regexp_boundary_char_map_labels)]) for kv_charpoint in regexp_boundary_char_map.items()]
[dict([(label, kv_charpoint[index]) for index, label in enumerate(regexp_boundary_char_map_labels)]) for kv_charpoint in regexp_boundary_char_map.items()]
[dict([pair for pair in [(label, kv_charpoint[index]) for index, label in enumerate(regexp_boundary_char_map_labels)]]) for kv_charpoint in regexp_boundary_char_map.items()]
regexp_boundary_char_map_labels
regexp_boundary_char_map_labels
dict(one=1) + dict(two=2)
dict(one=1).update(dict(two=2))
d1=dict(one=1)
d1=dict(one=1)
d2=dict(two=2)
d1.update(d2) or d1
globals()
globals().get('foobarz', -42)
globals().update(dict(foobarz=42)) or globals().get('foobarz', -42)
globals().get('foobarz', -42)
[dict([(label, kv_charpoint[index]) for index, label in enumerate(regexp_boundary_char_map_labels)]) for kv_charpoint in regexp_boundary_char_map.items()]
[dict([(label, kv_charpoint[index]) for index, label in enumerate(regexp_boundary_char_map_labels)]) for kv_charpoint in regexp_boundary_char_map.items()]
[data for data in [dict([(label, kv_charpoint[index]) for index, label in enumerate(regexp_boundary_char_map_labels)]) for kv_charpoint in regexp_boundary_char_map.items()]]
[data.update(dict([(fn.__name__, fn(data['codepoint'])[2:].rjust(width, '0')) ])) or data  for fn, width in [(hex, 2), (oct, 4), (bin, 8)] ) for data in [dict([(label, kv_charpoint[index]) for index, label in enumerate(regexp_boundary_char_map_labels)]) for kv_charpoint in regexp_boundary_char_map.items()]]
[data.update(dict([(fn.__name__, fn(data['codepoint'])[2:].rjust(width, '0')) ])) or data  for fn, width in [(hex, 2), (oct, 4), (bin, 8)] ) for data in [dict([(label, kv_charpoint[index]) for index, label in enumerate(regexp_boundary_char_map_labels)])] for kv_charpoint in regexp_boundary_char_map.items()]
[data.update(dict([(fn.__name__, fn(data['codepoint'])[2:].rjust(width, '0')) ])) or data  for fn, width in [(hex, 2), (oct, 4), (bin, 8)] ) ) for data in [dict([(label, kv_charpoint[index]) for index, label in enumerate(regexp_boundary_char_map_labels)])] for kv_charpoint in regexp_boundary_char_map.items()]
