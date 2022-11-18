import collections
import queue
import sys


def unique_results(transitions, original):
    result = set()
    discards = 0

    for i in range(len(original)):
        prefix = original[:i]
        tail = original[i:]

        for key in transitions:
            if tail.startswith(key):
                for dest in transitions[key]:
                    final = prefix + dest + tail[len(key):]
                    result.add(final)

    return result


def minimum_distance(transitions, begin, target):
    pq = queue.PriorityQueue()
    pq.put((len(begin), begin, 0))

    while not pq.empty():
        _, word, distance = pq.get()

        if word == target:
            return distance

        for neighbor in unique_results(transitions, word):
            pq.put((len(neighbor), neighbor, distance + 1))


if __name__ == "__main__":
    transitions = collections.defaultdict(set)
    reverse_transitions = collections.defaultdict(set)
    with open(sys.argv[1], "r") as f:
        for line in f.readlines():
            line = line.strip()
            parts = line.split(" => ")

            if len(parts) < 2:
                molecule = line
            else:
                transitions[parts[0]].add(parts[1])
                reverse_transitions[parts[1]].add(parts[0])

    part1 = len(unique_results(transitions, molecule))
    print(f"Unique transitions: {part1}")
    part2 = minimum_distance(reverse_transitions, molecule, 'e')
    print(f"Minimum transitions: {part2}")
