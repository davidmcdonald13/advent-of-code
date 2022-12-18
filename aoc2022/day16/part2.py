import collections
import sys


class Memo:
    def __init__(self, valves, tunnels):
        self.valves = valves
        self.tunnels = tunnels
        self.memo = dict()

    def dfs(self, person, elephant, open_valves, mins_remaining):
        key = (min(person, elephant), max(person, elephant), tuple(open_valves), mins_remaining)

        if key in self.memo:
            return self.memo[key]

        result = self._dfs(person, elephant, open_valves, mins_remaining)
        self.memo[key] = result
        return result

    def _dfs(self, person, elephant, open_valves, mins_remaining):
        if mins_remaining <= 0:
            return 0

        current_rate = sum([self.valves[valve] for valve in open_valves])
        if len(open_valves) == len(self.tunnels) - 1:
            return current_rate * mins_remaining

        result = 0

        if person not in open_valves and self.valves[person] > 0:
            new_open_valves = {x for x in open_valves}
            new_open_valves.add(person)

            for elephant_dest in self.tunnels[elephant]:
                result = max(result, current_rate + self.dfs(person, elephant_dest, new_open_valves, mins_remaining - 1))

        if elephant not in open_valves and self.valves[elephant] > 0:
            new_open_valves = {x for x in open_valves}
            new_open_valves.add(elephant)

            for person_dest in self.tunnels[person]:
                result = max(result, current_rate + self.dfs(person_dest, elephant, new_open_valves, mins_remaining - 1))

        if person != elephant and person not in open_valves and elephant not in open_valves and self.valves[person] > 0 and self.valves[elephant] > 0:
            new_open_valves = {x for x in open_valves}
            new_open_valves.add(person)
            new_open_valves.add(elephant)
            result = max(result, current_rate + self.dfs(person, elephant, new_open_valves, mins_remaining - 1))

        for person_dest in self.tunnels[person]:
            for elephant_dest in self.tunnels[elephant]:
                result = max(result, current_rate + self.dfs(person_dest, elephant_dest, open_valves, mins_remaining - 1))

        return result


def get_min_distance(start, end, tunnels):
    queue = [(start, 0)]
    visited = set()

    while queue:
        curr, dist = queue[0]
        queue = queue[1:]

        if curr == end:
            return dist

        if curr in visited:
            continue
        visited.add(curr)

        for neighbor in tunnels[curr]:
            queue.append((neighbor, dist + 1))


def bypass_zero_nodes(valves, tunnels):
    result = collections.defaultdict(list)

    nodes = [k for k in valves.keys() if valves[k] > 0]
    nodes.append("AA")

    for i, start in enumerate(nodes):
        for end in nodes[i+1:]:
            if start == end:
                continue

            distance = get_min_distance(start, end, tunnels)

            result[start].append((end, distance))
            result[end].append((start, distance))
    return result


def run(filename):
    valves, tunnels = parse_file(filename)

    #tunnels = bypass_zero_nodes(valves, tunnels)

    memo = Memo(valves, tunnels)
    return memo.dfs("AA", "AA", set(), 26)


def parse_file(filename):
    valves = dict()
    tunnels = dict()

    with open(filename, 'r') as f:
        for line in f.readlines():
            tokens = line.split()

            node = tokens[1]

            rate = int(tokens[4][:-1].split("=")[1])

            paths = [tok.split(",")[0] for tok in tokens[9:]]

            valves[node] = rate
            tunnels[node] = paths

    return valves, tunnels


if __name__ == "__main__":
    print(run(sys.argv[1]))
