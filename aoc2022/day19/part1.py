import queue
import sys


class Resources:
    def __init__(self, ore, clay, obsidian):
        self.ore = ore
        self.clay = clay
        self.obsidian = obsidian

    def __eq__(self, other):
        return self.ore == other.ore and self.clay == other.clay and self.obsidian == other.obsidian

    def __ge__(self, other):
        return self.ore >= other.ore and self.clay >= other.clay and self.obsidian >= other.obsidian

    def __add__(self, other):
        return Resources(self.ore + other.ore, self.clay + other.clay, self.obsidian + other.obsidian)

    def __sub__(self, other):
        return Resources(self.ore - other.ore, self.clay - other.clay, self.obsidian - other.obsidian)

    def __str__(self):
        return f"Resources(ore={self.ore}, clay={self.clay}, obsidian={self.obsidian})"


class State:
    def __init__(self, ore_robots, clay_robots, obsidian_robots, geode_robots, resources):
        self.ore_robots = ore_robots
        self.clay_robots = clay_robots
        self.obsidian_robots = obsidian_robots
        self.geode_robots = geode_robots
        self.resources = resources
        self._tuple = (self.ore_robots, self.clay_robots, self.obsidian_robots, self.geode_robots, self.resources.ore, self.resources.clay, self.resources.obsidian)

    def new_resources(self):
        return Resources(self.ore_robots, self.clay_robots, self.obsidian_robots)

    @staticmethod
    def default():
        return State(1, 0, 0, 0, Resources(0, 0, 0))

    def copy(self, ore_robots=None, clay_robots=None, obsidian_robots=None, geode_robots=None,
            resources=None):
        if ore_robots is None:
            ore_robots = self.ore_robots
        if clay_robots is None:
            clay_robots = self.clay_robots
        if obsidian_robots is None:
            obsidian_robots = self.obsidian_robots
        if geode_robots is None:
            geode_robots = self.geode_robots
        if resources is None:
            resources = self.resources

        return State(ore_robots, clay_robots, obsidian_robots, geode_robots, resources)

    def __hash__(self):
        return hash(self._tuple)

    def __eq__(self, other):
        return self._tuple == other._tuple

    def __lt__(self, other):
        return self._tuple < other._tuple

    def __str__(self):
        return str(self._tuple)


class Graph:
    def __init__(self, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost):
        self.ore_robot_cost = ore_robot_cost
        self.clay_robot_cost = clay_robot_cost
        self.obsidian_robot_cost = obsidian_robot_cost
        self.geode_robot_cost = geode_robot_cost

        self._costs = [self.ore_robot_cost, self.clay_robot_cost, self.obsidian_robot_cost, self.geode_robot_cost]

    # this has to ALWAYS be larger than the actual maximum possible, but closer to accurate is better
    def _get_theoretical_max(self, state, mins_remaining):
        result = 0
        while mins_remaining > 0:
            result += state.geode_robots

            new_resources = state.new_resources()

            can_build_ore = state.resources >= self.ore_robot_cost
            can_build_clay = state.resources >= self.clay_robot_cost
            can_build_obsidian = state.resources >= self.obsidian_robot_cost
            can_build_geode = state.resources >= self.geode_robot_cost

            if can_build_ore and can_build_clay and can_build_obsidian and can_build_geode:
                spent = Resources(
                        min([x.ore for x in self._costs]),
                        min([x.clay for x in self._costs]),
                        min([x.obsidian for x in self._costs]),
                        )
            else:
                spent = Resources(0, 0, 0)

            state = state.copy(resources = state.resources - spent + new_resources,
                               ore_robots = state.ore_robots + int(can_build_ore),
                               clay_robots = state.clay_robots + int(can_build_clay),
                               obsidian_robots = state.obsidian_robots + int(can_build_obsidian),
                               geode_robots = state.geode_robots + int(can_build_geode),
                               )
            mins_remaining -= 1
        return result

    def solve(self, state, mins_remaining):
        pq = queue.PriorityQueue()
        visited = set()

        result = 0

        theoretical_max = self._get_theoretical_max(state, mins_remaining)
        pq.put((-theoretical_max, 0, state, mins_remaining))
        while not pq.empty():
            _theoretical_max, _accrued, state, mins_remaining = pq.get()
            theoretical_max = -_theoretical_max
            accrued = -_accrued
            #print(f"{theoretical_max}, {accrued}, {state}, {mins_remaining}, {result}")

            if theoretical_max <= result:
                return result

            if mins_remaining == 0:
                result = max(theoretical_max, result)
                continue

            if (state, mins_remaining) in visited:
                continue
            visited.add((state, mins_remaining))

            new_resources = state.new_resources()

            future_states = list()

            # do nothing
            future_states.append(state.copy(resources = state.resources + new_resources))

            # build ore robot
            if state.resources >= self.ore_robot_cost:
                future_states.append(state.copy(resources = state.resources - self.ore_robot_cost + new_resources,
                                                ore_robots = state.ore_robots + 1,
                                                ))

            # build clay robot
            if state.resources >= self.clay_robot_cost:
                future_states.append(state.copy(resources = state.resources - self.clay_robot_cost + new_resources,
                                                clay_robots = state.clay_robots + 1,
                                                ))

            # build obsidian robot
            if state.resources >= self.obsidian_robot_cost:
                future_states.append(state.copy(resources = state.resources - self.obsidian_robot_cost + new_resources,
                                                obsidian_robots = state.obsidian_robots + 1,
                                                ))

            # build geode robot
            if state.resources >= self.geode_robot_cost:
                future_states.append(state.copy(resources = state.resources - self.geode_robot_cost + new_resources,
                                                geode_robots = state.geode_robots + 1,
                                                ))

            for future_state in future_states:
                upper_bound = self._get_theoretical_max(future_state, mins_remaining - 1)
                pq.put((-(upper_bound + accrued + state.geode_robots), -(accrued + state.geode_robots),
                       future_state, mins_remaining - 1))

        assert(False)

def solve(line):
    tokens = line.split()

    ore_robot_cost = Resources(int(tokens[6]), 0, 0)
    clay_robot_cost = Resources(int(tokens[12]), 0, 0)
    obsidian_robot_cost = Resources(int(tokens[18]), int(tokens[21]), 0)
    geode_robot_cost = Resources(int(tokens[27]), 0, int(tokens[30]))

    graph = Graph(ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost)
    result = graph.solve(State.default(), 24)

    return result

def run(filename):
    result = 0
    with open(filename, 'r') as f:
        for i, line in enumerate(f.readlines()):
            value = solve(line)
            print(f"blueprint {i+1} found {value} geodes")
            result += (i + 1) * value

    return result


if __name__ == "__main__":
    print(run(sys.argv[1]))
