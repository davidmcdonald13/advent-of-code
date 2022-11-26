import sys


def test():
    assert(distance("R2, L3") == 5)
    assert(distance("R2, R2, R2") == 2)
    assert(distance("R5, L5, R5, R3") == 12)

    assert(visited_twice("R8, R4, R4, R8") == 4)


def raw_distance(distances):
    return abs(distances[0] - distances[2]) + abs(distances[1] - distances[3])


def abs_distance(distances):
    return (distances[0] - distances[2], distances[1] - distances[3])


def distance(moves, first_duplicate=False):
    # north, east, south, west
    distances = [0, 0, 0, 0]
    orientation = 0

    visited = set()

    for move in moves.split(", "):
        magnitude = int(move[1:])

        if move[0] == "R":
            orientation += 1
        else:
            orientation -= 1

        # mod 4
        orientation &= 3

        for _ in range(magnitude):
            distances[orientation] += 1

            location = abs_distance(distances)
            if first_duplicate and location in visited:
                return raw_distance(distances)
            visited.add(location)

    return raw_distance(distances)


def visited_twice(moves):
    return distance(moves, first_duplicate=True)


def run(filename):
    with open(filename, 'r') as f:
        val = f.read().strip()
        print("distance:", distance(val))
        print("visited_twice:", visited_twice(val))


if __name__ == "__main__":
    test()
    run(sys.argv[1])
