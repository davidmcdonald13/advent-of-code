import sys


class Jets:
    def __init__(self, chars):
        self.chars = chars
        self.index = 0

    def next(self, count=1):
        result = self.chars[self.index:self.index+count]
        self.index = (self.index + count) % len(self.chars)

        if len(result) < count:
            result += self.chars[:self.index]

        return result


class Row:
    def __init__(self, x, y, initial_jets=""):
        x = x + {
                "": 0,
                ">>>>": 1,
                ">>><": 0,
                ">><>": 1,
                ">><<": -1,
                "><>>": 1,
                "><><": 0,
                "><<>": 0,
                "><<<": -2,
                "<>>>": 1,
                "<>><": 0,
                "<><>": 0,
                "<><<": -2,
                "<<>>": 0,
                "<<><": -2,
                "<<<>": -1,
                "<<<<": -2,
                }[initial_jets]
        self.x = x
        self.y = y
        self.points = [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]
        self.highest_point = y

class Plus:
    def __init__(self, x, y, initial_jets=""):
        x = x + {
                "": 0,
                ">>>>": 2,
                ">>><": 1,
                ">><>": 2,
                ">><<": 0,
                "><>>": 2,
                "><><": 0,
                "><<>": 0,
                "><<<": -2,
                "<>>>": 2,
                "<>><": 0,
                "<><>": 0,
                "<><<": -2,
                "<<>>": 0,
                "<<><": -2,
                "<<<>": -1,
                "<<<<": -2,
                }[initial_jets]
        self.x = x
        self.y = y
        self.points = [(x + 1, y), (x, y + 1), (x + 1, y + 1), (x + 2, y + 1), (x + 1, y + 2)]
        self.highest_point = y + 2

class Angle:
    def __init__(self, x, y, initial_jets=""):
        x = x + {
                "": 0,
                ">>>>": 2,
                ">>><": 1,
                ">><>": 2,
                ">><<": 0,
                "><>>": 2,
                "><><": 0,
                "><<>": 0,
                "><<<": -2,
                "<>>>": 2,
                "<>><": 0,
                "<><>": 0,
                "<><<": -2,
                "<<>>": 0,
                "<<><": -2,
                "<<<>": -1,
                "<<<<": -2,
                }[initial_jets]
        self.x = x
        self.y = y
        self.points = [(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1), (x + 2, y + 2)]
        self.highest_point = y + 2

class Pole:
    def __init__(self, x, y, initial_jets=""):
        x = x + {
                "": 0,
                ">>>>": 4,
                ">>><": 2,
                ">><>": 2,
                ">><<": 0,
                "><>>": 2,
                "><><": 0,
                "><<>": 0,
                "><<<": -2,
                "<>>>": 2,
                "<>><": 0,
                "<><>": 0,
                "<><<": -2,
                "<<>>": 0,
                "<<><": -2,
                "<<<>": -1,
                "<<<<": -2,
                }[initial_jets]
        self.x = x
        self.y = y
        self.points = [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]
        self.highest_point = y + 3

class Cube:
    def __init__(self, x, y, initial_jets=""):
        x = x + {
                "": 0,
                ">>>>": 3,
                ">>><": 2,
                ">><>": 2,
                ">><<": 0,
                "><>>": 2,
                "><><": 0,
                "><<>": 0,
                "><<<": -2,
                "<>>>": 2,
                "<>><": 0,
                "<><>": 0,
                "<><<": -2,
                "<<>>": 0,
                "<<><": -2,
                "<<<>": -1,
                "<<<<": -2,
                }[initial_jets]
        self.x = x
        self.y = y
        self.points = [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)]
        self.highest_point = y + 1


class Grid:
    def __init__(self):
        self.width = 7
        self.grid = [[False for _ in range(self.width)] for _ in range(10)]
        self.y_offset = 0

    def is_open(self, x, y):
        if x < 0 or x >= self.width or y < self.y_offset:
            return False

        real_y = y - self.y_offset

        if real_y >= len(self.grid):
            needed_rows = real_y - len(self.grid) + 1
            self.grid.extend([[False for _ in range(self.width)] for _ in range(needed_rows)])

        return not self.grid[y - self.y_offset][x]

    def insert(self, shape):
        for x, y in shape.points:
            real_y = y - self.y_offset
            if real_y >= len(self.grid):
                needed_rows = real_y - len(self.grid) + 1
                self.grid.extend([[False for _ in range(self.width)] for _ in range(needed_rows)])
            if real_y < 0:
                continue

            self.grid[real_y][x] = True
            if all(self.grid[real_y]):
                self.clean()

    def clean(self):
        for i in range(len(self.grid) - 1, -1, -1):
            if all(self.grid[i]):
                self.y_offset += i + 1
                self.grid = self.grid[i+1:]
                return

    def print(self):
        for row in self.grid[::-1]:
            buf = "|"
            for val in row:
                if val:
                    buf += "#"
                else:
                    buf += "."
            buf += "|"
            print(buf)

def run(filename, num):
    with open(filename, 'r') as f:
        jets = Jets(f.read().strip())

    shapes = [Row, Plus, Angle, Pole, Cube]
    grid = Grid()

    pile_height = 0

    jet_index_rollovers = list()
    heights = list()

    #for i in range(1_000_000_000_000):
    for i in range(num):
        #if i % 1000 == 0:
        #    grid.clean()

        jet_index_beginning = jets.index

        shape = shapes[i % len(shapes)]
        initial_jets = jets.next(count=4)
        #print(initial_jets)
        rock = shape(2, pile_height, initial_jets)
        #print(rock.points)

        while True:
            down_candidate = shape(rock.x, rock.y - 1)

            if all(grid.is_open(x, y) for x, y in down_candidate.points):
                rock = down_candidate
            else:
                grid.insert(rock)
                pile_height = max(pile_height, rock.highest_point + 1)
                break

            if jets.next() == "<":
                jet_candidate = shape(rock.x - 1, rock.y)
            else:
                jet_candidate = shape(rock.x + 1, rock.y)

            if all(grid.is_open(x, y) for x, y in jet_candidate.points):
                rock = jet_candidate

        if jets.index < jet_index_beginning:
            jet_index_rollovers.append(i)
            heights.append(pile_height)

    height_diffs = list()
    jet_index_rollover_diffs = list()
    for i in range(len(heights) - 1):
        height_diffs.append(heights[i+1] - heights[i])
        jet_index_rollover_diffs.append(jet_index_rollovers[i+1] - jet_index_rollovers[i])

    print(f"first jet index rollover: {jet_index_rollovers[0]}")
    print(jet_index_rollover_diffs)
    print()
    print(height_diffs)

    #grid.print()
    return pile_height


if __name__ == "__main__":
    print(run(sys.argv[1], int(sys.argv[2])))
