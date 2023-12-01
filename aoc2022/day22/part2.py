import collections
import sys


class Node:
    def __init__(self, grid_coords, val):
        self.grid_coords = grid_coords
        self.cube_coords = None
        self.face_vector = None
        self.val = val
        self.visited = False

    def __hash__(self):
        return hash(self.grid_coords)

    def __eq__(self, other):
        return self.grid_coords == other.grid_coords

    #def __str__(self):
    #    return f"Node(row={self.row}, col={self.col}, val={self.val})"


def tuple2_add(a, b):
    return (a[0] + b[0], a[1] + b[1])

def tuple3_add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])

def tuple3_sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def process_input(filename):
    with open(filename, 'r') as f:
        grid, directions = f.read().rstrip().split("\n\n")

    width = max(len(row) for row in grid)

    root = None
    graph_2d = dict()
    for i, row in enumerate(grid.split("\n")):
        for j, val in enumerate(row):
            if val == " ":
                continue
            coords = (i, j)
            if root is None:
                root = coords
            node = Node(coords, val)
            graph_2d[coords] = node

    cube_length = (len(graph_2d) // 6) ** 0.5

    graph_3d = collections.defaultdict(list)
    queue = [(root, (0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, -1))]
    while queue:
        grid_coords, cube_coords, row_diff, col_diff, face_vector = queue.pop(0)

        if grid_coords not in graph_2d:
            continue

        node = graph_2d[grid_coords]
        if node.cube_coords is not None:
            continue

        node.cube_coords = cube_coords
        node.face_vector = face_vector

        graph_3d[cube_coords].append(node)

        # no need to ever move up on the 2d grid
        # move down on 2d grid
        down_grid_coords = tuple2_add(grid_coords, (1, 0))
        if down_grid_coords[0] % cube_length == 0:
            queue.append((down_grid_coords,
                          cube_coords,
                          tuple3_sub((0, 0, 0), face_vector), # LOL this can't be right -- right?
                          col_diff,
                          row_diff # LOL this can't be right -- right?
                          ))
        else:
            queue.append((down_grid_coords,
                          tuple3_add(cube_coords, row_diff),
                          row_diff,
                          col_diff,
                          face_vector))

        left_grid_coords = tuple2_add(grid_coords, (0, -1))
        # look at the *current* coordinates to see if we're crossing a face barrier
        if grid_coords[1] % cube_length == 0:
            queue.append((left_grid_coords,
                          cube_coords,
                          row_diff,
                          face_vector, # LOL this can't be right -- right?
                          tuple3_sub((0, 0, 0), col_diff), # LOL this can't be right -- right?
                          ))
        else:
            queue.append((left_grid_coords,
                          tuple3_sub(cube_coords, col_diff),
                          row_diff,
                          col_diff,
                          face_vector))

        right_grid_coords = tuple2_add(grid_coords, (0, 1))
        if right_grid_coords[1] % cube_length == 0:
            queue.append((right_grid_coords,
                          cube_coords,
                          row_diff,
                          tuple3_sub((0, 0, 0), face_vector), # LOL this can't be right -- right?
                          col_diff, # LOL this can't be right -- right?
                          ))
        else:
            queue.append((right_grid_coords,
                          tuple3_add(cube_coords, col_diff),
                          row_diff,
                          col_diff,
                          face_vector))

    for i, row in enumerate(grid.split("\n")):
        buf = ""
        for j, val in enumerate(row):
#            """
#            if (i, j) in graph_2d:
#                buf += str({(0, 0, -1): 1,
#                            (0, 0, 1): 5,
#                            (1, 0, 0): 4,
#                            (-1, 0, 0): 2,
#                            (0, 1, 0): 6,
#                            (0, -1 ,0): 3,
#                            }[graph_2d[(i, j)].face_vector])
#            else:
#                buf += " "
#            """
            if (i, j) not in graph_2d:
                buf += " "
                continue
            if len(graph_3d[graph_2d[(i, j)].cube_coords]) >= 3:
                buf += "#"
                continue
            if len(graph_3d[graph_2d[(i, j)].cube_coords]) == 1:
                buf += "."
                continue

            faces = {(0, 0, -1): 1,
                    (0, 0, 1): 4,
                    (1, 0, 0): 3,
                    (-1, 0, 0): 6,
                    (0, 1, 0): 2,
                    (0, -1 ,0): 5,
                    }
            pair = tuple(sorted([faces[node.face_vector] for node in graph_3d[graph_2d[(i, j)].cube_coords]]))

            buf += {(1, 2): 'A',
                    (1, 3): 'B',
                    (1, 5): 'C',
                    (1, 6): 'D',
                    (2, 3): 'E',
                    (2, 4): 'F',
                    (2, 6): 'G',
                    (3, 4): 'H',
                    (3, 5): 'I',
                    (4, 5): 'J',
                    (4, 6): 'K',
                    (5, 6): 'L',
                    }[pair]
        print(buf)

    for coords, node in graph_2d.items():
        assert(node.cube_coords is not None)
        is_on_row_edge = coords[0] % cube_length == 0 or (coords[0] + 1) % cube_length == 0
        is_on_col_edge = coords[1] % cube_length == 0 or (coords[1] + 1) % cube_length == 0

        assert(len(graph_3d[node.cube_coords]) == 1 + int(is_on_row_edge) + int(is_on_col_edge))

    return graph_2d, graph_3d, directions


def get_distance(directions):
    i = 0
    while i < len(directions) and directions[i] not in "LR":
        i += 1
    return int(directions[:i]), directions[i:]


def _right_turn(face_vector, orientation):
    if face_vector == (1, 0, 0):
        return {(0, 1, 0): (0, 0, 1),
                (0, -1, 0): (0, 0, -1),
                (0, 0, 1): (0, -1, 0),
                (0, 0, -1): (0, 1, 0),
                }[orientation]
    elif face_vector == (-1, 0, 0):
        return {(0, 1, 0): (0, 0, -1),
                (0, -1, 0): (0, 0, 1),
                (0, 0, 1): (0, 1, 0),
                (0, 0, -1): (0, -1, 0),
                }[orientation]
    elif face_vector == (0, 1, 0):
        return {(1, 0, 0): (0, 0, -1),
                (-1, 0, 0): (0, 0, 1),
                (0, 0, 1): (1, 0, 0),
                (0, 0, -1): (-1, 0, 0),
                }[orientation]
    elif face_vector == (0, -1, 0):
        return {(1, 0, 0): (0, 0, 1),
                (-1, 0, 0): (0, 0, -1),
                (0, 0, 1): (-1, 0, 0),
                (0, 0, -1): (1, 0, 0),
                }[orientation]
    elif face_vector == (0, 0, 1):
        return {(1, 0, 0): (0, 1, 0),
                (-1, 0, 0): (0, -1, 0),
                (0, 1, 0): (-1, 0, 0),
                (0, -1, 0): (1, 0, 0),
                }[orientation]
    elif face_vector == (0, 0, -1):
        return {(1, 0, 0): (0, -1, 0),
                (-1, 0, 0): (0, 1, 0),
                (0, 1, 0): (1, 0, 0),
                (0, -1, 0): (-1, 0, 0),
                }[orientation]


def get_turn(directions, orientation, curr_node):
    if not directions:
        return orientation, directions

    old = orientation

    # just make three rights to go left
    count = 1 + 2 * int(directions[0] == "L")
    for _ in range(count):
        orientation = _right_turn(curr_node.face_vector, orientation)

    #print(f"turned {directions[0]} (face={curr_node.face_vector}): {old} to {orientation}")
    return orientation, directions[1:]


def get_node_across_faces(graph, curr_node, orientation):
    dest_node = get_node(graph, curr_node.cube_coords, orientation)
    dest_orientation = tuple3_sub((0, 0, 0), curr_node.face_vector)
    #print(f"face_vector={curr_node.face_vector}, orientation={orientation}, result_face_vector={dest_node.face_vector}, result_orientation={dest_orientation}")
    return dest_node, dest_orientation


def get_node(graph, coords, face_vector):
    for candidate in graph[coords]:
        if candidate.face_vector == face_vector:
            return candidate
    assert(False)


#def orientation_to_direction(orientation, node):



def follow_directions(graph, directions, graph_2d):
    curr = get_node(graph, (0, 0, 0), (0, 0, -1))
    orientation = (0, 1, 0)
    while directions:
        assert(curr.val == ".")
        curr.visited = True
        distance, directions = get_distance(directions)
        #print(f"distance={distance}")
        #print(f"coords={curr.grid_coords}, orientation={orientation}")

        for _ in range(distance):
            dest_coords = tuple3_add(curr.cube_coords, orientation)
            dest_orientation = orientation
            if dest_coords in graph:
                dest_node = get_node(graph, dest_coords, curr.face_vector)
            else:
                dest_node, dest_orientation = get_node_across_faces(graph, curr, orientation)
            if dest_node.val == "#":
                #if dest_orientation != orientation:
                    #print("Ran into an over-the-edge wall!")
                    #print(curr.grid_coords, orientation)
                break
            curr = dest_node
            assert(curr.val == ".")
            curr.visited = True
            orientation = dest_orientation

        #print_graph(graph_2d)
        #input()

        #print(f"turning {directions[0]}, orientation={orientation}, face_vector={curr.face_vector}")
        orientation, directions = get_turn(directions, orientation, curr)
        #print(f"updated: orientation={orientation}, face_vector={curr.face_vector}")

    print(f"orientation={orientation}")

    return curr.grid_coords[0], curr.grid_coords[1], None#orientation_to_direction(orientation, curr)


def print_graph(graph):
    width = max(x[1] for x in graph.keys())
    height = max(x[0] for x in graph.keys())

    for i in range(height + 1):
        buf = ""
        for j in range(width + 1):
            if (i, j) not in graph:
                buf += " "
                continue
            if graph[(i, j)].visited:
                buf += "X"
            else:
                buf += graph[(i, j)].val
        print(buf)


def run(filename):
    graph_2d, graph_3d, directions = process_input(filename)

    row, col, facing = follow_directions(graph_3d, directions, graph_2d)
    #print_graph(graph_2d)
    print(f"row={row}, col={col}, facing={facing}")

    return 1000 * (row + 1) + 4 * (col + 1) #+ facing


if __name__ == "__main__":
    print(run(sys.argv[1]))
    print("answer is greater than 13535")
