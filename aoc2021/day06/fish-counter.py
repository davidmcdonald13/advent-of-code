import sys

filename = sys.argv[1]
days = int(sys.argv[2])

with open(filename, "r") as f:
    initial_str = f.readline().split(",")

states = dict()
for init in initial_str:
    val = int(init)
    states[val] = states.get(val, 0) + 1

for day in range(days):
    new = dict()
    for val, count in states.items():
        if val == 0:
            new[6] = new.get(6, 0) + count
            new[8] = new.get(8, 0) + count
        else:
            new[val-1] = new.get(val - 1, 0) + count
    states = new

print(sum(states.values()))
