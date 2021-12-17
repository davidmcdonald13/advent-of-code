def lookandsay(s):
    def _lookandsay(s):
        if not s:
            return "", ""

        i = 1
        while i < len(s) and s[i] == s[0]:
            i += 1

        return str(i) + s[0], s[i:]

    result = list()
    while s:
        res, s = _lookandsay(s)
        result.append(res)

    return "".join(result)

assert(lookandsay("1") == "11")
assert(lookandsay("11") == "21")
assert(lookandsay("21") == "1211")
assert(lookandsay("1211") == "111221")
assert(lookandsay("111221") == "312211")

val = "1113122113"
for i in range(50):
    print(i, len(val))
    val = lookandsay(val)

print(len(val))
