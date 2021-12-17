from collections import defaultdict


LETTERS = "abcdefghijklmnopqrstuvwxyz"
assert(len(LETTERS) == 26)
assert(len(set(LETTERS)) == 26)

ILLEGAL_CHARS = ["i", "o", "l"]

original = "cqjxxyzz"
raw = [ord(c) - ord('a') for c in original]

while True:
    for i in range(len(raw) - 1, -1, -1):
        raw[i] = (raw[i] + 1) % 26
        if raw[i] != 0:
            break

    string = "".join([chr(c + ord('a')) for c in raw])
    print("checking:", string)

    is_illegal = False
    for illegal in ILLEGAL_CHARS:
        if illegal in string:
            is_illegal = True
            break

    contains_triad = False
    for i in range(len(LETTERS) - 2):
        check = LETTERS[i:i+3]
        if check in string:
            contains_triad = True
            break

    contains_duplicates = False
    duplicates = defaultdict(list)
    for i in range(len(string) - 1):
        if string[i] == string[i+1]:
            duplicates[string[i]].append(i)
            if len(duplicates) >= 2 or duplicates[string[i]][-1] - duplicates[string[i]][0] > 1:
                contains_duplicates = True
                break

    if not is_illegal and contains_triad and contains_duplicates:
        print(string)
        exit()
