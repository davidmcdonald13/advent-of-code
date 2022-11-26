import sys


def is_valid(nums):
    nums = sorted(nums)
    return nums[0] + nums[1] > nums[2]


def run(filename, horizontal=True):
    nums = list()
    with open(filename, "r") as f:
        for line in f.readlines():
            line = line.strip()
            nums.append([int(x) for x in line.split()])

    result = 0
    if horizontal:
        for row in nums:
            result += is_valid(row)
    else:
        for i in range(0, len(nums), 3):
            for j in range(3):
                result += is_valid([nums[i][j], nums[i+1][j], nums[i+2][j]])

    return result


if __name__ == "__main__":
    print(run(sys.argv[1], horizontal=True))
    print(run(sys.argv[1], horizontal=False))
