def container_fill(amount, containers):
    if amount == 0:
        return {0: 1}
    if not containers:
        return dict()

    not_include = container_fill(amount, containers[1:])
    include = container_fill(amount - containers[0], containers[1:])

    result = dict()
    for key, value in not_include.items():
        result[key] = value
    for key, value in include.items():
        if key + 1 in result:
            result[key + 1] += value
        else:
            result[key + 1] = value

    return result

print("sample:", container_fill(25, [20, 15, 10, 5, 5]))
print("real:", container_fill(150, [43,
3,
4,
10,
21,
44,
4,
6,
47,
41,
34,
17,
17,
44,
36,
31,
46,
9,
27,
38]))
