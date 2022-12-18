import sys


class Row:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.points = [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]
        self.highest_point = y

class Plus:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.points = [(x + 1, y), (x, y + 1), (x + 1, y + 1), (x + 2, y + 1), (x + 1, y + 2)]
        self.highest_point = y + 2

class Angle:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.points = [(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1), (x + 2, y + 2)]
        self.highest_point = y + 2

class Pole:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.points = [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]
        self.highest_point = y + 3

class Cube:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.points = [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)]
        self.highest_point = y + 1


class Grid:
    def __init__(self):
        self.width = 7
        self.grid = [[False for _ in range(self.width)] for _ in range(1000)]
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
            self.grid[y - self.y_offset][x] = True

    def clean(self):
        for i in range(len(self.grid) - 1, -1, -1):
            if all(self.grid[i]):
                self.y_offset += i + 1
                self.grid = self.grid[i+1:]
                return


def run(filename, num):
    with open(filename, 'r') as f:
        jets = f.read().strip()

    jet_index = 0

    shapes = [Row, Plus, Angle, Pole, Cube]
    grid = Grid()

    pile_height = 0

    for i in range(num):
        #if i % 100 == 0:
        #    grid.clean()

        shape = shapes[i % len(shapes)]
        rock = shape(2, pile_height + 3)

        while True:
            if jets[jet_index] == "<":
                jet_candidate = shape(rock.x - 1, rock.y)
            else:
                jet_candidate = shape(rock.x + 1, rock.y)
            jet_index = (jet_index + 1) % len(jets)

            if all(grid.is_open(x, y) for x, y in jet_candidate.points):
                rock = jet_candidate

            down_candidate = shape(rock.x, rock.y - 1)

            if all(grid.is_open(x, y) for x, y in down_candidate.points):
                rock = down_candidate
            else:
                grid.insert(rock)
                pile_height = max(pile_height, rock.highest_point + 1)
                break

    return pile_height


if __name__ == "__main__":
    print(run(sys.argv[1], int(sys.argv[2])))
