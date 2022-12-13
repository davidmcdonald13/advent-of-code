import functools
import json
import sys


def is_valid(left, right):
    if type(left) == int and type(right) == int:
        if left < right:
            return True
        elif left == right:
            return None
        return False

    if type(left) == int:
        left = [left]
    if type(right) == int:
        right = [right]

    for i in range(len(left)):
        if i >= len(right):
            return False
        recurse = is_valid(left[i], right[i])
        if recurse:
            return True
        elif recurse == False:
            return False

    if len(left) < len(right):
        return True

    return None


def arr_cmp(left, right):
    result = is_valid(json.loads(left), json.loads(right))
    if result:
        result = -1
    else:
        result = 1
    return result


def main(filename):
    dividers = ["[[2]]", "[[6]]"]
    vals = ["[[2]]", "[[6]]"]

    with open(filename, 'r') as f:
        for line in f.readlines():
            line = line.strip()
            if line:
                vals.append(line)

    ordered = sorted(vals, key=functools.cmp_to_key(arr_cmp))

    return (ordered.index(dividers[0]) + 1) * (ordered.index(dividers[1]) + 1)


if __name__ == "__main__":
    print(main(sys.argv[1]))
