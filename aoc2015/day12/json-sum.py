import json
import sys


def _json_sum(obj):
    if type(obj) == list:
        return sum([_json_sum(x) for x in obj])
    elif type(obj) == dict:
        if "red" in obj.values():
            return 0
        return sum([_json_sum(x) for x in obj.values()])
    elif type(obj) == int:
        return obj
    else:
        return 0

def json_sum(s):
    obj = json.loads(s)
    return _json_sum(obj)
"""
    result = 0
    prev_dash = False
    current = 0
    for c in s:
        if c in "0123456789":
            # we're looking at a number
            current *= 10
            current += int(c)
            if prev_dash:
                current = -abs(current)
        else:
            # we're looking at a not-number
            result += current
            current = 0
            prev_dash = c == "-"
            if prev_dash:
                print("FOUND A DASH!")
                exit()

    return result
"""


assert(json_sum("[1,2,3]") == 6)
assert(json_sum('{"a":2,"b":4}') == 6)
assert(json_sum("[[[3]]]") == 3)
assert(json_sum('{"a":{"b":4},"c":-1}') == 3)
assert(json_sum('{"a":[-1,1]}') == 0)
assert(json_sum('[-1,{"a":1}]') == 0)
assert(json_sum("[]") == 0)
assert(json_sum("{}") == 0)

filename = sys.argv[1]
with open(filename, 'r') as f:
    print(json_sum(f.read()))
