import sys

filename = sys.argv[1]
total_seconds = int(sys.argv[2])

result = 0
spots = list()
with open(filename, 'r') as f:
    for line in f.readlines():
        tokens = line.strip().split()

        name = tokens[0]
        speed = int(tokens[3])
        run_time = int(tokens[6])
        rest_time = int(tokens[-2])

        location = list()
        for second in range(1, total_seconds + 1):
            period = run_time + rest_time
            full_periods = second // period

            remainder = min(run_time, second % period)

            distance = (full_periods * run_time + remainder) * speed
            location.append(distance)

        spots.append(location)

print("farthest distance:", max([location[-1] for location in spots]))

points = [0 for _ in spots]
for i in range(len(spots[0])):
    max_val = max([location[i] for location in spots])
    for j in range(len(points)):
        if spots[j][i] == max_val:
            points[j] += 1

print("most points:", max(points))
