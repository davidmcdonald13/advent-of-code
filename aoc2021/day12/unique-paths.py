import sys

from collections import defaultdict


def is_small(node):
    return ord('a') <= ord(node[0]) <= ord('z')


def dfs(node, target, graph, disallowed):
    #print(node, target, graph, disallowed)
    if node in disallowed:
        return list()
    if node == target:
        return [[target]]

    new_disallowed = set(disallowed)
    if is_small(node):
        new_disallowed.add(node)

    result = list()
    for dest in graph[node]:
        subresult = dfs(dest, target, graph, new_disallowed)
        for x in subresult:
            result.append([node] + x)

    return result


def main(filename):
    graph = defaultdict(set)
    with open(filename, "r") as f:
        for line in f.readlines():
            a, b = line.strip().split("-")
            graph[a].add(b)
            graph[b].add(a)

    result = dfs("start", "end", graph, set())
    print("path_count:", len(result))


if __name__ == "__main__":
    filename = sys.argv[1]
    main(filename)
