import collections
import sys


def decode(filename, optimizer):
    counts = collections.defaultdict(lambda: collections.defaultdict(lambda: 0))
    with open(filename, "r") as f:
        for line in f.readlines():
            line = line.strip()
            for i, c in enumerate(line):
                counts[i][c] += 1

    result = list()
    for key in sorted(counts.keys()):
        optimal_letter = None
        optimal_val = None
        for letter in counts[key].keys():
            if optimal_val is None or optimizer(counts[key][letter], optimal_val):
                optimal_val = counts[key][letter]
                optimal_letter = letter

        result.append(optimal_letter)

    return "".join(result)


if __name__ == "__main__":
    print(decode(sys.argv[1], lambda a, b: a > b))
    print(decode(sys.argv[1], lambda a, b: a < b))
