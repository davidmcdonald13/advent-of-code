import hashlib


def test():
    assert(hash_check("abc") == "18f47a30")
    print("tested correctly!")


def hash_check(seed):
    result = list()
    index = 0
    while len(result) < 8:
        joined = seed + str(index)
        res = hashlib.md5(bytes(joined)).hexdigest()

        if res.startswith("00000"):
            result.append(res[5])

        index += 1

    return "".join(result)


if __name__ == "__main__":
    test()
    print(hash_check("reyedfim"))
