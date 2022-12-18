import collections
import sys


class Memo:
    def __init__(self, valves, tunnels):
        self.valves = valves
        self.tunnels = tunnels
        self.memo = dict()

    def dfs(self, person_dest, person_remaining, elephant_dest, elephant_remaining, open_valves, mins_remaining):
        key = (person_dest, person_remaining, elephant_dest, elephant_remaining, tuple(sorted(list(open_valves))), mins_remaining)

        if key in self.memo:
            return self.memo[key]

        result = self._dfs(person_dest, person_remaining, elephant_dest, elephant_remaining, open_valves, mins_remaining)
        self.memo[key] = result
        return result

    def _dfs(self, person_dest, person_remaining, elephant_dest, elephant_remaining, open_valves, mins_remaining):
        if mins_remaining <= 0:
            return 0

        check = person_dest == "DD" and elephant_dest == "JJ" and mins_remaining == 25
        if check:
            print("called once")

        if check:
            print(f"dfs({person_dest}, {person_remaining}, {elephant_dest}, {elephant_remaining}, {open_valves}, {mins_remaining})")

        if person_remaining > 0 and elephant_remaining > 0:
            return self.dfs(person_dest, person_remaining - 1, elephant_dest,
                            elephant_remaining - 1, open_valves, mins_remaining - 1)

        result = 0

        if person_remaining == 0:
            # need to assign person NOTE might also need to assign elephant
            if self.valves[person_dest] > 0 and person_dest not in open_valves:
                new_open_valves = {x for x in open_valves}
                new_open_valves.add(person_dest)

                guaranteed = self.valves[person_dest] * (mins_remaining - 1)
            else:
                new_open_valves = open_valves
                guaranteed = 0

            found_dest = False
            for next_person, cost in self.tunnels[person_dest]:
                if next_person in new_open_valves or next_person == elephant_dest or next_person == "AA":
                    if check:
                        print(f"won't send person to {next_person}")
                    continue

                found_dest = True
                if check:
                    print(f"found destination: {next_person}")

                result = max(result, guaranteed + self.dfs(next_person, cost, elephant_dest, elephant_remaining, new_open_valves, mins_remaining))

            if not found_dest:
                if check:
                    print(f"no valid destinations, no need to recurse")

                if elephant_dest in new_open_valves:
                    # all valves are open
                    if check:
                        print(f"all valves are open!")

                    return guaranteed
                else:
                    # elephant is headed to the last valve
                    if check:
                        print(f"elephant headed to the last valve")
                    return guaranteed + self.valves[elephant_dest] * max(0, mins_remaining - elephant_remaining - 1)
        else:
            # need to assign elephant
            return self.dfs(elephant_dest, elephant_remaining, person_dest, person_remaining, open_valves, mins_remaining)

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
    print(len(tunnels))

    memo = Memo(valves, tunnels)
    return memo.dfs("AA", 0, "AA", 0, set(), 26)


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
