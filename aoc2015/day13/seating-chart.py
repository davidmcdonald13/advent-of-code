import sys

from collections import defaultdict

filename = sys.argv[1]

raw_graph = defaultdict(set)
with open(filename, 'r') as f:
    for line in f.readlines():
        tokens = line.strip()[:-1].split()

        a = tokens[0]
        b = tokens[-1]

        direction = tokens[2]
        score = int(tokens[3])
        if direction == "lose":
            score = -score

        raw_graph[a].add((b, score))

graph = defaultdict(set)
people = list(raw_graph.keys())
for i, a in enumerate(people):
    for b in people[i+1:]:
        full_cost = 0
        for person, cost in raw_graph[a]:
            if person == b:
                full_cost += cost
                break
        for person, cost in raw_graph[b]:
            if person == a:
                full_cost += cost
                break

        graph[(a, b)] = full_cost
        graph[(b, a)] = full_cost
        print(a, b, full_cost)

me = "myself"
for person in people:
    graph[(me, person)] = 0
    graph[(person, me)] = 0
people.append(me)

result = -99999999999999999999999999999999999
queue = [[people[0]]]
while queue:
    chart = queue.pop(0)
    if len(chart) == len(people):
        full_cost = graph[(chart[0], chart[-1])]
        for i in range(len(chart) - 1):
            full_cost += graph[(chart[i], chart[i+1])]

        result = max(result, full_cost)
        if result == full_cost:
            print(chart, full_cost)
        continue

    for person in people:
        if person not in chart:
            queue.append(chart + [person])

print(result)
