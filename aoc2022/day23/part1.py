import collections
import sys


class Elf:
    def __init__(self, x, y, val):
        self.x = x
        self.y = y
        self.direction = 0
        self.val = val

    def get_neighbors(self):
        result = list()
        for check_x in range(self.x - 1, self.x + 2):
            for check_y in range(self.y - 1, self.y + 2):
                if check_x == self.x and check_y == self.y:
                    continue
                result.append((check_x, check_y))
        return result

    def get_danger_zone(self, offset):
        direction = (self.direction + offset) % 4
        if direction == 0:
            # look north
            return [(self.x - 1, self.y + 1), (self.x, self.y + 1), (self.x + 1, self.y + 1)]
        elif direction == 1:
            # look south
            return [(self.x - 1, self.y - 1), (self.x, self.y - 1), (self.x + 1, self.y - 1)]
        elif direction == 2:
            # look west
            return [(self.x - 1, self.y - 1), (self.x - 1, self.y), (self.x - 1, self.y + 1)]
        else:
            # look east
            return [(self.x + 1, self.y - 1), (self.x + 1, self.y), (self.x + 1, self.y + 1)]

    def get_destination(self, offset):
        direction = (self.direction + offset) % 4
        if direction == 0:
            # go north
            return (self.x, self.y + 1)
        elif direction == 1:
            # go south
            return (self.x, self.y - 1)
        elif direction == 2:
            # go west
            return (self.x - 1, self.y)
        else:
            # go east
            return (self.x + 1, self.y)


def parse_input(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

        elves = dict()
        for row, line in enumerate(lines):
            for col, val in enumerate(line):
                if val == "#":
                    x = col
                    y = len(lines) - row

                    elves[(x, y)] = Elf(x, y, len(elves) + 1)
    return elves


def move_elves(elves):
    before = len(elves)
    proposed = collections.defaultdict(list)
    for coords, elf in elves.items():
        found_destination = False
        if any(neighbor in elves for neighbor in elf.get_neighbors()):
            for offset in range(4):
                if not any(neighbor in elves for neighbor in elf.get_danger_zone(offset)):
                    proposed[elf.get_destination(offset)].append(elf)
                    found_destination = True
                    break
        if not found_destination:
            proposed[coords].append(elf)
            found_destination = True
        elf.direction += 1
        assert(found_destination)

    result = dict()
    for new_coords, elves in proposed.items():
        if len(elves) > 1:
            for elf in elves:
                result[(elf.x, elf.y)] = elf
        else:
            result[new_coords] = elves[0]
            elves[0].x = new_coords[0]
            elves[0].y = new_coords[1]

    assert(len(result) == before)
    return result


def get_result(elves):
    xs = [coords[0] for coords in elves]
    ys = [coords[1] for coords in elves]

    return (max(xs) + 1 - min(xs)) * (max(ys) + 1 - min(ys)) - len(elves)


def print_elves(elves):
    xs = [coords[0] for coords in elves]
    ys = [coords[1] for coords in elves]

    print(f"x_min={min(xs)}")
    for y in range(max(ys), min(ys) - 1, -1):
        buf = ""
        if 0 <= y < 10:
            buf = " "
        buf += f"{y}: "
        for x in range(min(xs), max(xs) + 1):
            if (x, y) in elves:
                buf += "#"
            else:
                buf += "."
        print(buf)


def run(filename):
    elves = parse_input(filename)

    for _ in range(10):
        #print_elves(elves)
        #input()
        elves = move_elves(elves)

    return get_result(elves)


if __name__ == "__main__":
    print(run(sys.argv[1]))
