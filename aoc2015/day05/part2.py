import sys

filename = sys.argv[1]

def is_nice(s):
    pairs = {s[:2]: [0]}
    contains_duplicate_pair = False
    contains_valid_three = False
    for i in range(2, len(s)):
        pair = s[i-1:i+1]
        if pair not in pairs:
            pairs[pair] = list()
        pairs[pair].append(i-1)
        if len(pairs[pair]) > 2 or (len(pairs[pair]) == 2 and pairs[pair][-1] - pairs[pair][-2] > 1):
            contains_duplicate_pair = True

        if s[i] == s[i-2]:
            contains_valid_three = True
    return contains_duplicate_pair and contains_valid_three

count = 0
with open(filename, "r") as f:
    for line in f.readlines():
        line = line.strip()
        if is_nice(line):
            count += 1

print("nice strings:", count)
