import sys

filename = sys.argv[1]

N = 1000
lights = [[False for _ in range(N)] for row in range(N)]
brightness = [[0 for _ in range(N)] for row in range(N)]

def get_coords(s):
    tokens = s.split(",")
    return int(tokens[0]), int(tokens[1])

with open(filename, 'r') as f:
    for line in f.readlines():
        tokens = line.strip().split()

        a = tokens[-3]
        b = tokens[-1]
        operation = tokens[-4]

        a_row, a_col = get_coords(a)
        b_row, b_col = get_coords(b)

        for row in range(min(a_row, b_row), max(a_row, b_row) + 1):
            for col in range(min(a_col, b_col), max(a_col, b_col) + 1):
                if operation == "on":
                    lights[row][col] = True
                    brightness[row][col] += 1
                elif operation == "off":
                    lights[row][col] = False
                    brightness[row][col] = max(0, brightness[row][col] - 1)
                elif operation == "toggle":
                    lights[row][col] = not lights[row][col]
                    brightness[row][col] += 2

count = 0
for row in lights:
    for val in row:
        count += int(val)

print("lit:", count)

brightness_sum = 0
for row in brightness:
    brightness_sum += sum(row)

print("brightness:", brightness_sum)
