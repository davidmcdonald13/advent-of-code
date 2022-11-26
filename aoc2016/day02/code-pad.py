import sys


def get_code(filename, traditional_keys=True):
    result = list()

    if traditional_keys:
        keys = [[1, 2, 3],
                [4, 5, 6],
                [7, 8, 9],
               ]
        row, col = 1, 1
    else:
        keys = [[None, None,   1, None, None],
                [None,    2,   3,    4, None],
                [   5,    6,   7,    8,    9],
                [None,  "A", "B",  "C", None],
                [None, None, "D", None, None],
               ]
        row, col = 2, 0

    with open(filename, "r") as f:
        for line in f.readlines():
            line = line.strip()

            for c in line:
                if c == "U":
                    row -= 1
                    if row < 0 or keys[row][col] is None:
                        row += 1
                elif c == "D":
                    row += 1
                    if row >= len(keys) or keys[row][col] is None:
                        row -= 1
                elif c == "L":
                    col -= 1
                    if col < 0 or keys[row][col] is None:
                        col += 1
                else:
                    col += 1
                    if col >= len(keys[0]) or keys[row][col] is None:
                        col -= 1

            result.append(keys[row][col])

    result = [str(x) for x in result]
    return "".join(result)


if __name__ == "__main__":
    print(get_code(sys.argv[1]))
    print(get_code(sys.argv[1], traditional_keys=False))
