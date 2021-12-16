import sys

filename = sys.argv[1]

wrapping_paper = 0
ribbon = 0

with open(filename, 'r') as f:
    for line in f.readlines():
        dims = [int(x) for x in line.strip().split("x")]
        dims.sort()

        a = dims[0] * dims[1]
        b = dims[1] * dims[2]
        c = dims[0] * dims[2]
        wrapping_paper += 2 * (a + b + c) + a

        ribbon += 2 * (dims[0] + dims[1]) + (dims[0] * dims[1] * dims[2])

print("wrapping paper:", wrapping_paper)
print("ribbon:", ribbon)
