import sys

from collections import defaultdict


def is_small(node):
    return ord('a') <= ord(node[0]) <= ord('z')


def is_super_disallowed(node):
    return node in ["start", "end"]


def dfs(node, target, graph, disallowed, override_disallowed):
    new_override_disallowed = override_disallowed
    if node in disallowed:
        if override_disallowed and not is_super_disallowed(node):
            new_override_disallowed = False
        else:
            return list()
    if node == target:
        return [[target]]

    new_disallowed = set(disallowed)
    if is_small(node):
        new_disallowed.add(node)

    result = list()
    for dest in graph[node]:
        subresult = dfs(dest, target, graph, new_disallowed, new_override_disallowed)
        for x in subresult:
            result.append([node] + x)

    return result


def main(filename, override_disallowed):
    graph = defaultdict(set)
    with open(filename, "r") as f:
        for line in f.readlines():
            a, b = line.strip().split("-")
            graph[a].add(b)
            graph[b].add(a)

    result = dfs("start", "end", graph, set(), override_disallowed)

    return len(result)


if __name__ == "__main__":
    override_disallowed = sys.argv[1] == "True"
    filenames = sys.argv[2:]
    for filename in filenames:
        print(filename, "result:", main(filename, override_disallowed))
