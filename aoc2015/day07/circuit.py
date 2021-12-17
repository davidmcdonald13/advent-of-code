import sys

from collections import defaultdict

values = dict()
queue = list()
base_instructions = list()
operators = defaultdict(list)

with open(sys.argv[1], 'r') as f:
    for line in f.readlines():
        operation, output = line.strip().split(" -> ")

        if "AND" in operation:
            root1, root2 = operation.split(" AND ")
            roots = (root1, root2)
            operation = "AND"
        elif "OR" in operation:
            root1, root2 = operation.split(" OR ")
            roots = (root1, root2)
            operation = "OR"
        elif "LSHIFT" in operation:
            root1, root2 = operation.split(" LSHIFT ")
            roots = (root1, root2)
            operation = "LSHIFT"
        elif "RSHIFT" in operation:
            root1, root2 = operation.split(" RSHIFT ")
            roots = (root1, root2)
            operation = "RSHIFT"
        elif "NOT" in operation:
            root = operation.split("NOT ")[1]
            roots = (root,)
            operation = "NOT"
        else:
            roots = (operation,)
            operation = "SET"

        obj = (roots, operation, output)
        for root in roots:
            try:
                literal = int(root)
                values[root] = literal
            except:
                operators[root].append(obj)

        if operation == "SET":
            queue.append(obj)
            base_instructions.append(obj)

while queue:
    obj = queue.pop(0)
    roots, operation, output = obj

    if output in values:
        continue

    if not all([root in values for root in roots]):
        queue.append(obj)
        continue

    values[output] = {
            "AND": lambda: values[roots[0]] & values[roots[1]],
            "OR": lambda: values[roots[0]] | values[roots[1]],
            "LSHIFT": lambda: values[roots[0]] << values[roots[1]],
            "RSHIFT": lambda: values[roots[0]] >> values[roots[1]],
            "NOT": lambda: values[roots[0]] ^ 0xFFFF,
            "SET": lambda: values[roots[0]],
            }[operation]()

    queue.extend(operators[output])

print("a:", values['a'])
a_value = values['a']
values[str(a_value)] = a_value

to_remove = set()
for key in values:
    try:
        int(key)
    except:
        to_remove.add(key)

for key in to_remove:
    values.pop(key)

queue.append(((str(a_value),), "SET", 'b'))
queue.extend(base_instructions)
while queue:
    obj = queue.pop(0)
    roots, operation, output = obj

    if output in values:
        continue

    if not all([root in values for root in roots]):
        queue.append(obj)
        continue

    values[output] = {
            "AND": lambda: values[roots[0]] & values[roots[1]],
            "OR": lambda: values[roots[0]] | values[roots[1]],
            "LSHIFT": lambda: values[roots[0]] << values[roots[1]],
            "RSHIFT": lambda: values[roots[0]] >> values[roots[1]],
            "NOT": lambda: values[roots[0]] ^ 0xFFFF,
            "SET": lambda: values[roots[0]],
            }[operation]()

    queue.extend(operators[output])

print("a:", values['a'])
