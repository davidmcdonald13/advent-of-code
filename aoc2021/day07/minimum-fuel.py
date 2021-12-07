import sys

filename = sys.argv[1]

with open(filename, "r") as f:
    positions = [int(x) for x in f.readline().split(",")]

result = list()
for target in range(min(positions), max(positions) + 1):
    result.append(sum([abs(target - x) for x in positions]))

print(min(result))
