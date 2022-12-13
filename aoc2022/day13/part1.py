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


def block_valid(block):
    lines = block.split()

    return is_valid(json.loads(lines[0]), json.loads(lines[1]))


def main(filename):
    with open(filename, 'r') as f:
        blocks = f.read().split("\n\n")

    result = 0
    for i, block in enumerate(blocks):
        if block_valid(block):
            result += i + 1

    return result


if __name__ == "__main__":
    print(main(sys.argv[1]))
