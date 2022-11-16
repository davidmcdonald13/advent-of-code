import collections
import sys


def run(transitions, original):
    result = set()

    for i in range(len(original)):
        prefix = original[:i]
        tail = original[i:]

        for key in transitions:
            if tail.startswith(key):
                for dest in transitions[key]:
                    result.add(prefix + dest + tail[len(key):])

    return len(result)


if __name__ == "__main__":
    transitions = collections.defaultdict(set)
    with open(sys.argv[1], 'r') as f:
        for line in f.readlines():
            line = line.strip()
            parts = line.split(" => ")

            if len(parts) < 2:
                molecule = line
            else:
                transitions[parts[0]].add(parts[1])

    print(run(transitions, molecule))
