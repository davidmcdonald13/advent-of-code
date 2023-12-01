import sys


map_10_to_5 = {
    "2": 2,
    "1": 1,
    "0": 0,
    "-": -1,
    "=": -2,
}


def base_5_to_10(s):
    result = 0
    mult = 1
    while s:
        result += mult * map_10_to_5[s[-1]]
        s = s[:-1]
        mult *= 5

    return result


def base_10_to_5(num):
    result = ""
    while num:
        c = ["0", "1", "2", "=", "-"][num % 5]
        result = c + result

        num = (num - map_10_to_5[c]) // 5

    return result


def run(filename):
    with open(filename, 'r') as f:
        total = sum(base_5_to_10(line.strip()) for line in f.readlines())

    print(f"decimal sum: {total}")

    return base_10_to_5(total)


def test():
    assert(base_5_to_10("12") == 7)
    assert(base_5_to_10("21") == 11)
    assert(base_5_to_10("1=") == 3)
    assert(base_5_to_10("1=-0-2") == 1747)
    assert(base_5_to_10("12111") == 906)
    assert(base_5_to_10("2=0=") == 198)
    assert(base_5_to_10("2=01") == 201)
    assert(base_5_to_10("111") == 31)
    assert(base_5_to_10("20012") == 1257)
    assert(base_5_to_10("112") == 32)
    assert(base_5_to_10("1=-1=") == 353)
    assert(base_5_to_10("1-12") == 107)
    assert(base_5_to_10("122") == 37)

    assert(base_10_to_5(1) == "1")
    assert(base_10_to_5(2) == "2")
    assert(base_10_to_5(3) == "1=")
    assert(base_10_to_5(4) == "1-")
    assert(base_10_to_5(5) == "10")
    assert(base_10_to_5(6) == "11")
    assert(base_10_to_5(7) == "12")
    assert(base_10_to_5(8) == "2=")
    assert(base_10_to_5(9) == "2-")
    assert(base_10_to_5(10) == "20")
    assert(base_10_to_5(15) == "1=0")
    assert(base_10_to_5(20) == "1-0")
    assert(base_10_to_5(2022) == "1=11-2")
    assert(base_10_to_5(12345) == "1-0---0")
    assert(base_10_to_5(314159265) == "1121-1110-1=0")


if __name__ == "__main__":
    test()
    print(run(sys.argv[1]))
