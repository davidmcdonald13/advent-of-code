import sys


class Node:
    def __init__(self, row, col, val):
        self.row = row
        self.col = col
        self.val = val
        self.neighbors = [None for _ in range(4)]

    def __hash__(self):
        return hash((self.row, self.col))

    def __eq__(self, other):
        return self.row == other.row and self.col == other.col

    def __str__(self):
        return f"Node(row={self.row}, col={self.col}, val={self.val})"


def process_input(filename):
    with open(filename, 'r') as f:
        grid, directions = f.read().rstrip().split("\n\n")

    width = max(len(row) for row in grid)

    graph = dict()

    for i, row in enumerate(grid.split("\n")):
        for j, val in enumerate(row):
            if val == " ":
                continue
            node = Node(i, j, val)
            graph[(i, j)] = node

    # fill out left/right neighbors
    for row, col in graph:
        left = graph[(row, col)]
        if (row, col + 1) in graph:
            right = graph[(row, col + 1)]
        else:
            right_col = min(coords[1] for coords in graph if coords[0] == row)
            right = graph[(row, right_col)]

        left.neighbors[0] = right
        right.neighbors[2] = left

    # fill out up/down neighbors
    for row, col in graph:
        up = graph[(row, col)]
        if (row + 1, col) in graph:
            down = graph[(row + 1, col)]
        else:
            down_row = min(coords[0] for coords in graph if coords[1] == col)
            down = graph[(down_row, col)]

        up.neighbors[1] = down
        down.neighbors[3] = up

    for node in graph.values():
        for x in node.neighbors:
            if x is None:
                print(node)
            assert(x is not None)

    return graph, directions


def get_distance(directions):
    i = 0
    while i < len(directions) and directions[i] not in "LR":
        i += 1
    return int(directions[:i]), directions[i:]


def get_turn(directions):
    if not directions:
        return 0, directions
    if directions[0] == "L":
        return -1, directions[1:]
    else:
        return 1, directions[1:]


def follow_directions(graph, directions):
    start_col = min(coords[1] for coords in graph if coords[0] == 0)

    curr = (0, start_col)
    orientation = 0
    while directions:
        distance, directions = get_distance(directions)

        for _ in range(distance):
            dest = graph[curr].neighbors[orientation]
            if dest.val == "#":
                break
            curr = (dest.row, dest.col)

        orientation_delta, directions = get_turn(directions)
        orientation = (orientation + orientation_delta) % 4

    return curr[0], curr[1], orientation


def run(filename):
    graph, directions = process_input(filename)

    row, col, facing = follow_directions(graph, directions)

    return 1000 * (row + 1) + 4 * (col + 1) + facing


if __name__ == "__main__":
    print(run(sys.argv[1]))
