import sys


class Grid:
    def __init__(self, x_min, x_max, y_min, y_max, z_min, z_max):
        self.grid = [[[None for _ in range(z_min, z_max + 1)] for __ in range(y_min, y_max + 1)] for ___ in range(x_min, x_max + 1)]
        self.x_offset = x_min
        self.y_offset = y_min
        self.z_offset = z_min

    def _oob_check(self, real_x, real_y, real_z):
        return real_x < 0 or real_x >= len(self.grid) or real_y < 0 or real_y >= len(self.grid[0]) or real_z < 0 or real_z >= len(self.grid[0][0])

    def update(self, coords, value):
        real_x = coords[0] - self.x_offset
        real_y = coords[1] - self.y_offset
        real_z = coords[2] - self.z_offset

        if self._oob_check(real_x, real_y, real_z):
            return

        self.grid[real_x][real_y][real_z] = value

    def get(self, coords, default):
        real_x = coords[0] - self.x_offset
        real_y = coords[1] - self.y_offset
        real_z = coords[2] - self.z_offset

        if self._oob_check(real_x, real_y, real_z):
            return default

        return self.grid[real_x][real_y][real_z]


def get_neighbors(x, y, z):
    for offset in [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]]:
        yield [x + offset[0], y + offset[1], z + offset[2]]


def run(filename):
    cubes = list()

    with open(filename, 'r') as f:
        for line in f.readlines():
            cubes.append([int(x) for x in line.strip().split(",")])

    all_x = [cube[0] for cube in cubes]
    all_y = [cube[1] for cube in cubes]
    all_z = [cube[2] for cube in cubes]

    grid = Grid(min(all_x) - 1, max(all_x) + 1,
                min(all_y) - 1, max(all_y) + 1,
                min(all_z) - 1, max(all_z) + 1)

    for cube in cubes:
        grid.update(cube, False)

    queue = [[min(all_x) - 1, min(all_y) - 1, min(all_z) - 1]]

    while queue:
        coords = queue[0]
        queue = queue[1:]

        curr = grid.get(coords, "default_value")
        if curr is not None:
            continue

        grid.update(coords, True)

        queue.extend(list(get_neighbors(coords[0], coords[1], coords[2])))

    result = 0
    for x, y, z in cubes:
        for neighbor in get_neighbors(x, y, z):
            if grid.get(neighbor, False):
                result += 1
    return result



if __name__ == "__main__":
    print(run(sys.argv[1]))
