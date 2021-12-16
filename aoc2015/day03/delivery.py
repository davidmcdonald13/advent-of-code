import sys

filename = sys.argv[1]
santa_count = int(sys.argv[2])

visited = set()
santa_index = 0
locations = [(0, 0) for _ in range(santa_count)]
visited.add(locations[0])

with open(filename, 'r') as f:
    for line in f.readlines():
        for c in line.strip():
            location = locations[santa_index]
            if c == "^":
                location = (location[0] + 1, location[1])
            elif c == "v":
                location = (location[0] - 1, location[1])
            elif c == ">":
                location = (location[0], location[1] + 1)
            elif c == "<":
                location = (location[0], location[1] - 1)
            visited.add(location)
            locations[santa_index] = location
            santa_index = (santa_index + 1) % santa_count

print("houses visited:", len(visited))
