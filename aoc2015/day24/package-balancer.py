import queue
import sys


def product(arr):
    result = 1
    for x in arr:
        result *= x
    return result


def read_file(filename):
    values = list()
    with open(filename, 'r') as f:
        for line in f.readlines():
            values.append(int(line.strip()))
    return values


def overlap(x, y):
    for val in x:
        if val in y:
            return True
    return False

class Finder:
    def __init__(self):
        self.memo = dict()

    # return a list of lists where all lists sum to target
    def _get_all_candidates(self, all_values, target):
        assert(target >= 0)

        if not all_values:
            if target == 0:
                return [[]]
            return []

        if target == 0:
            return [[]]

        result = list()
        if all_values[0] <= target:
            for res in self.get_all_candidates(all_values[1:], target - all_values[0]):
                result.append([all_values[0]] + res)

        result.extend(self.get_all_candidates(all_values[1:], target))

        return result


    def get_all_candidates(self, all_values, target):
        key = (len(all_values), target)
        if key in self.memo:
            return self.memo[key]
        result = self._get_all_candidates(all_values, target)
        self.memo[key] = result
        return result


def is_valid(all_partitions, selected, overlap_cache, partition_count):
    if len(selected) == partition_count:
        return True

    for index in range(selected[-1] + 1, len(all_partitions)):
        overlaps_previous = False
        for other in selected:
            if (other, index) not in overlap_cache:
                overlap_cache[(other, index)] = overlap(all_partitions[other], all_partitions[index])

            if overlap_cache[(other, index)]:
                overlaps_previous = True
                break

        if overlaps_previous:
            continue

        if is_valid(all_partitions, selected + [index], overlap_cache, partition_count):
            return True

    return False


def run(filename, partition_count):
    weights = read_file(filename)

    # assume it's divisible by partition_count
    target = sum(weights) // partition_count

    finder = Finder()
    all_partitions = [set(x) for x in finder.get_all_candidates(weights, target)]
    all_partitions.sort(key=lambda x: (len(x), product(x)))

    overlap_cache = dict()

    N = len(all_partitions)
    for i in range(N):
        if is_valid(all_partitions, [i], overlap_cache, partition_count):
            return product(all_partitions[i])


if __name__ == '__main__':
    print(run(sys.argv[1], int(sys.argv[2])))
