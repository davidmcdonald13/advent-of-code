import collections
import sys


class Memo:
    def __init__(self, valves, tunnels, permitted_valves):
        self.valves = valves
        self.tunnels = tunnels
        self.permitted_valves = permitted_valves
        self.memo = dict()

    def dfs(self, curr, open_valves, mins_remaining):
        key = (curr, tuple(sorted(list(open_valves))), mins_remaining)

        if key in self.memo:
            return self.memo[key]

        result = self._dfs(curr, open_valves, mins_remaining)
        self.memo[key] = result
        return result

    def _dfs(self, curr, open_valves, mins_remaining):
        if mins_remaining <= 0:
            return 0

        current_rate = sum([self.valves[valve] for valve in open_valves])

        if self.valves[curr] > 0:
            new_open_valves = {x for x in open_valves}
            new_open_valves.add(curr)
            mins_remaining -= 1
            accrued = current_rate
            current_rate += self.valves[curr]
        else:
            new_open_valves = open_valves
            accrued = 0
        result = accrued + current_rate * mins_remaining

        for dest, cost in self.tunnels[curr]:
            if dest in self.permitted_valves and dest not in new_open_valves:
                result = max(result, accrued + current_rate * min(cost, mins_remaining) + self.dfs(dest, new_open_valves, mins_remaining - cost))

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

    tunnels = bypass_zero_nodes(valves, tunnels)

    ordered_tunnels = list(tunnels.keys())

    result = 0
    for mask in range(1, 1 << (len(tunnels) - 1)):
        left = list()
        right = list()
        for i, tunnel in enumerate(tunnels):
            if mask & (1 << i):
                left.append(tunnel)
            else:
                right.append(tunnel)

        left_res = Memo(valves, tunnels, left).dfs("AA", set(), 26)
        right_res = Memo(valves, tunnels, right).dfs("AA", set(), 26)

        if left_res + right_res > result:
            print(f"new result: {left_res + right_res}, {bin(mask)}")
            print(f"\tleft={left}, right={right}")
        result = max(result, left_res + right_res)
    #left_res = Memo(valves, tunnels, ["JJ", "BB", "CC"]).dfs("AA", set(), 26)
    #right_res = Memo(valves, tunnels, ["DD", "HH", "EE"]).dfs("AA", set(), 26)
    #result = left_res + right_res

    return result


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
