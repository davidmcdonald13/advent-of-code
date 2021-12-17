import sys

filename = sys.argv[1]

def get_len(line):
    code_len = len(line)
    mem_len = 0

    no_quotes = line[1:-1]
    i = 0
    while i < len(no_quotes):
        if no_quotes[i] == "\\":
            if no_quotes[i+1] == "x":
                i += 4
            else:
                i += 2
        else:
            i += 1
        mem_len += 1
    return code_len, mem_len


original_code_len = 0
original_mem_len = 0
new_code_len = 0

with open(filename, 'r') as f:
    for line in f.readlines():
        line = line.strip()
        this_code_len, this_mem_len = get_len(line)

        original_code_len += this_code_len
        original_mem_len += this_mem_len

        new_code_len += 2
        for c in line:
            if c == "\\" or c == "\"":
                new_code_len += 2
            else:
                new_code_len += 1


print("original code len:", original_code_len)
print("original mem len:", original_mem_len)
print("original diff:", original_code_len - original_mem_len)
print()
print("new code len:", new_code_len)
print("updated diff:", new_code_len - original_code_len)
