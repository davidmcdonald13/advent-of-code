import collections
import queue
import sys


class Blizzard:
    def __init__(self, row, col, is_horizontal, is_forward, grid_height, grid_width):
        self.row = row
        self.col = col
        self.is_horizontal = is_horizontal
        self.is_forward = is_forward
        self.grid_height = grid_height
        self.grid_width = grid_width

    def location_at_time(self, time):
        if self.is_forward:
            mult = 1
        else:
            mult = -1

        if self.is_horizontal:
            return (self.row, (self.col + time * mult) % self.grid_width)
        return ((self.row + time * mult) % self.grid_height, self.col)


def run(filename):
    blizzards = list()
    with open(filename, 'r') as f:
        lines = f.readlines()
        grid_height = len(lines) - 2
        for row, line in enumerate(lines):
            line = line.strip()
            grid_width = len(line) - 2
            if line.startswith("##") or line.endswith("##"):
                dest_row = row - 1
                continue
            for col, val in enumerate(line[1:]):
                dest_col = col - 1
                if val == ">":
                    blizzards.append(Blizzard(row - 1, col, True, True, grid_height, grid_width))
                elif val == "<":
                    blizzards.append(Blizzard(row - 1, col, True, False, grid_height, grid_width))
                elif val == "^":
                    blizzards.append(Blizzard(row - 1, col, False, False, grid_height, grid_width))
                elif val == "v":
                    blizzards.append(Blizzard(row - 1, col, False, True, grid_height, grid_width))

    horizontal_blizzards = collections.defaultdict(list)
    vertical_blizzards = collections.defaultdict(list)

    for blizzard in blizzards:
        if blizzard.is_horizontal:
            horizontal_blizzards[blizzard.row].append(blizzard)
        else:
            vertical_blizzards[blizzard.col].append(blizzard)

    print(f"grid_width={grid_width}, grid_height={grid_height}, dest_row={dest_row}, dest_col={dest_col}")

    coords_allowlist = [(dest_row, dest_col), (-1, 0)]
    visited = set()
    pq = queue.PriorityQueue()
    # priority, time, row, col, trip number
    pq.put((0, 0, -1, 0, 0))
    result = None
    while not pq.empty():
        priority, time, row, col, trip_number = pq.get()

        if (row, col) == coords_allowlist[trip_number & 1]:
            if trip_number == 2:
                return time
            trip_number += 1

        if (time, row, col, trip_number) in visited:
            continue
        visited.add((time, row, col, trip_number))

        if any(blizzard.location_at_time(time) == (row, col) for blizzard in horizontal_blizzards[row]) or any(blizzard.location_at_time(time) == (row, col) for blizzard in vertical_blizzards[col]):
            continue

        for row_diff, col_diff in [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)]:
            check_row = row + row_diff
            check_col = col + col_diff

            if check_row < 0 or check_row >= dest_row or check_col < 0 or check_col >= (dest_col + 1):
                if (check_row, check_col) not in coords_allowlist:
                    continue

            priority = time + 1 + (dest_row - check_row) + (dest_col - check_col) + (2 - trip_number) * (coords_allowlist[0][0] - coords_allowlist[1][0] + coords_allowlist[0][1] - coords_allowlist[1][1])

            pq.put((priority, time + 1, check_row, check_col, trip_number))

    print("no result found!")
    return None


if __name__ == "__main__":
    print(run(sys.argv[1]))
