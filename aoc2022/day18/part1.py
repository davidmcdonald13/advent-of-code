import sys


def run(filename):
    cubes = list()

    with open(filename, 'r') as f:
        for line in f.readlines():
            cubes.append([int(x) for x in line.strip().split(",")])

    result = 0
    for x, y, z in cubes:
        for offset in [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]]:
            if [x + offset[0], y + offset[1], z + offset[2]] not in cubes:
                result += 1
    return result


if __name__ == "__main__":
    print(run(sys.argv[1]))
