import hashlib
import sys

seed = sys.argv[1]
target = "".join(["0" for _ in range(int(sys.argv[2]))])

i = 1
while True:
    candidate = seed + str(i)
    result = hashlib.md5(bytes(candidate, "ascii")).hexdigest()
    if result.startswith(target):
        print(i)
        print(candidate)
        print(result)
        break
    i += 1
