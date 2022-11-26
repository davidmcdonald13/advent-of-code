import hashlib


def test():
    actual = hash_check("abc")
    if actual != "05ace8e3":
        print("actual:", actual)
        assert(actual == "05ace8e3")
    print("tested correctly!")


def prettify(vals):
    cleaned = list()
    for x in vals:
        if x is None:
            cleaned.append("_")
        else:
            cleaned.append(str(x))
    return "".join(cleaned)


def hash_check(seed):
    result = [None for _ in range(8)]
    populated = 0
    index = 0
    while populated < 8:
        joined = seed + str(index)
        res = hashlib.md5(bytes(joined)).hexdigest()

        if res.startswith("00000"):
            i = int(res[5], 16)
            print("Found index:", index, "which produces hash:", res)
            if i < len(result) and result[i] is None:
                result[i] = res[6]
                populated += 1
            print("current result:", prettify(result))

        index += 1

        if index % 1000000 == 0:
            print("checking index:", index)

    return prettify(result)


if __name__ == "__main__":
    test()
    print(hash_check("reyedfim"))
