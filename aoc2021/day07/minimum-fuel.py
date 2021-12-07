import sys

filename = sys.argv[1]
algo = sys.argv[2]

with open(filename, "r") as f:
    positions = [int(x) for x in f.readline().split(",")]

result = list()
for target in range(min(positions), max(positions) + 1):
    if algo == "linear":
        result.append(sum([abs(target - x) for x in positions]))
    elif algo == "quadratic":
        aggregator = 0
        for x in positions:
            diff = abs(target - x)
            aggregator += (diff ** 2 + diff) / 2
        result.append(aggregator)

print(min(result))
