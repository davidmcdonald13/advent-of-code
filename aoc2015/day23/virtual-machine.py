import sys


def run(filename, init_a=0):
    with open(filename, 'r') as f:
        instructions = []
        for line in f.readlines():
            instructions.append(line.strip().split())

    reg_a = init_a
    reg_b = 0
    ip = 0

    while ip < len(instructions):
        inst = instructions[ip]


        if inst[0] == 'hlf':
            if inst[1] == 'a':
                reg_a >>= 1
            else:
                reg_b >>= 1
            ip += 1

        elif inst[0] == 'tpl':
            if inst[1] == 'a':
                reg_a *= 3
            else:
                reg_b *= 3
            ip += 1

        elif inst[0] == 'inc':
            if inst[1] == 'a':
                reg_a += 1
            else:
                reg_b += 1
            ip += 1

        elif inst[0] == 'jmp':
            ip += int(inst[1])

        elif inst[0] == 'jie':
            if (inst[1] == 'a,' and reg_a % 2 == 0) or (inst[1] == 'b,' and reg_b % 2 == 0):
                ip += int(inst[2])
            else:
                ip += 1

        elif inst[0] == 'jio':
            if (inst[1] == 'a,' and reg_a == 1) or (inst[1] == 'b,' and reg_b == 1):
                ip += int(inst[2])
            else:
                ip += 1

    return reg_b


if __name__ == "__main__":
    print(run(sys.argv[1]))
    print(run(sys.argv[1], 1))
