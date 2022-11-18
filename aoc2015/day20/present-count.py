import sys


def min_house(threshold, presents_per_elf, max_houses=None):
    max_result = threshold // presents_per_elf + 1
    counts = [presents_per_elf for _ in range(max_result + 1)]
    # house i is at index i-1
    for elf in range(1, len(counts) + 1):
        if elf >= 2 and counts[elf - 2] >= threshold:
            return elf - 1

        if max_houses is None:
            max_index = len(counts)
        else:
            max_index = min(len(counts), (elf - 1) + elf * (max_houses + 1))
        for house in range(elf - 1, max_index, elf):
            counts[house] += elf * presents_per_elf


def part1():
    return min_house(29_000_000, 10)


def part2():
    return min_house(29_000_000, 11, max_houses=50)


if __name__ == "__main__":
    print(part1())
    print(part2())
