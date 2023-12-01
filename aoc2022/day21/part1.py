import sys


class Node:
    def eval(self, values):
        pass

class Leaf:
    def __init__(self, val):
        self.val = val

    def eval(self, values):
        return self.val

class Add:
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def eval(self, values):
        return values[self.left].eval(values) + values[self.right].eval(values)

class Subtract:
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def eval(self, values):
        return values[self.left].eval(values) - values[self.right].eval(values)

class Multiply:
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def eval(self, values):
        return values[self.left].eval(values) * values[self.right].eval(values)

class Divide:
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def eval(self, values):
        return values[self.left].eval(values) // values[self.right].eval(values)


def run(filename):
    vals = dict()
    with open(filename, 'r') as f:
        for line in f.readlines():
            name, ops = line.split(":")
            tokens = ops.split()

            if len(tokens) == 1:
                vals[name] = Leaf(int(tokens[0]))
            elif tokens[1] == "+":
                vals[name] = Add(tokens[0], tokens[2])
            elif tokens[1] == "-":
                vals[name] = Subtract(tokens[0], tokens[2])
            elif tokens[1] == "*":
                vals[name] = Multiply(tokens[0], tokens[2])
            elif tokens[1] == "/":
                vals[name] = Divide(tokens[0], tokens[2])

    return vals["root"].eval(vals)


if __name__ == "__main__":
    print(run(sys.argv[1]))
