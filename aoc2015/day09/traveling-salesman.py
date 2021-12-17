import sys

from collections import defaultdict

def dfs(node, graph, visited):
    new_visited = set()
    for x in visited:
        new_visited.add(x)
    new_visited.add(node)

    if len(new_visited) == len(graph):
        return 0, 0 

    min_distance = 999999999999999999999
    max_distance = 0
    for neighbor, cost in graph[node]:
        if neighbor in new_visited:
            continue
        this_min, this_max = dfs(neighbor, graph, new_visited)
        min_distance = min(min_distance, cost + this_min)
        max_distance = max(max_distance, cost + this_max)
    return min_distance, max_distance

def main(graph):
    min_distance = 9999999999999999999999999
    max_distance = 0

    for root in graph:
        this_min, this_max = dfs(root, graph, set())
        min_distance = min(min_distance , this_min)
        max_distance = max(max_distance , this_max)

    print("shortest distance:", min_distance)
    print("longest distance:", max_distance)

if __name__ == "__main__":
    filename = sys.argv[1]

    graph = defaultdict(set)
    with open(filename, 'r') as f:
        for line in f.readlines():
            tokens = line.strip().split()

            a = tokens[0]
            b = tokens[2]
            distance = int(tokens[-1])

            graph[a].add((b, distance))
            graph[b].add((a, distance))

    main(graph)
