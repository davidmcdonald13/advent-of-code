import sys


def product(arr):
    result = 1
    for x in arr:
        result *= max(x, 0)
    return result


def get_score(ingredients, counts):
    calories = 0
    for i, ingredient in enumerate(ingredients):
        calories += ingredient["calories"] * counts[i]
    if calories != 500:
        return -1

    sums = list()
    for key in ["capacity", "durability", "flavor", "texture"]:
        result = 0
        for i in range(len(ingredients)):
            result += ingredients[i][key] * counts[i]
        sums.append(result)
    return product(sums)


def optimizer(ingredients, already):
    if len(ingredients) - 1 == len(already):
        count = 100 - sum(already)
        return get_score(ingredients, already + [count])

    max_count = 100 - sum(already)
    result = 0
    for count in range(max_count + 1):
        counts = already + [count]
        result = max(result, optimizer(ingredients, counts))

    return result

if __name__ == "__main__":
    filename = sys.argv[1]

    ingredients = list()
    with open(filename, 'r') as f:
        for line in f.readlines():
            tokens = line.strip().split()

            traits = {
                    "capacity": int(tokens[2][:-1]),
                    "durability": int(tokens[4][:-1]),
                    "flavor": int(tokens[6][:-1]),
                    "texture": int(tokens[8][:-1]),
                    "calories": int(tokens[10]),
                    }
            ingredients.append(traits)

    print(optimizer(ingredients, []))
