import sys

filename = sys.argv[1]

INVALID_SUBSTRINGS = ["ab", "cd", "pq", "xy"]
VOWELS = "aeiou"

def is_nice(s):
    vowel_count = 0
    contains_duplicate = False
    for i, c in enumerate(s):
        if c in VOWELS:
            vowel_count += 1
        if i > 0 and s[i-1] == c:
            contains_duplicate = True
        if s[i-1:i+1] in INVALID_SUBSTRINGS:
            return False
    return vowel_count >= 3 and contains_duplicate

count = 0
with open(filename, "r") as f:
    for line in f.readlines():
        line = line.strip()
        if is_nice(line):
            count += 1

print("nice strings:", count)
