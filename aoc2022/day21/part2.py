import sys


class Node:
    def eval(self, values):
        pass

    def expect(self, values, target):
        pass

    def _safe_eval(self, values):
        left_eval, right_eval = None, None

        try:
            left_eval = values[self.left].eval(values)
        except:
            pass

        try:
            right_eval = values[self.right].eval(values)
        except:
            pass

        return left_eval, right_eval

class Leaf(Node):
    def __init__(self, val):
        self.val = val

    def eval(self, values):
        return self.val

    def expect(self, values, target):
        self.val = target

class Add(Node):
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def eval(self, values):
        return values[self.left].eval(values) + values[self.right].eval(values)

    def expect(self, values, target):
        left_eval, right_eval = self._safe_eval(values)

        if left_eval is None:
            values[self.left].expect(values, target - right_eval)
        elif right_eval is None:
            values[self.right].expect(values, target - left_eval)

class Subtract(Node):
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def eval(self, values):
        return values[self.left].eval(values) - values[self.right].eval(values)

    def expect(self, values, target):
        left_eval, right_eval = self._safe_eval(values)

        if left_eval is None:
            values[self.left].expect(values, target + right_eval)
        elif right_eval is None:
            values[self.right].expect(values, left_eval - target)

class Multiply(Node):
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def eval(self, values):
        return values[self.left].eval(values) * values[self.right].eval(values)

    def expect(self, values, target):
        left_eval, right_eval = self._safe_eval(values)

        if left_eval is None:
            values[self.left].expect(values, target // right_eval)
        elif right_eval is None:
            values[self.right].expect(values, target // left_eval)

class Divide(Node):
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def eval(self, values):
        return values[self.left].eval(values) // values[self.right].eval(values)

    def expect(self, values, target):
        left_eval, right_eval = self._safe_eval(values)

        if left_eval is None:
            values[self.left].expect(values, target * right_eval)
        elif right_eval is None:
            values[self.right].expect(values, left_eval // target)

class Equality(Node):
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def eval(self, values):
        left_eval, right_eval = self._safe_eval(values)

        if left_eval is None:
            values[self.left].expect(values, right_eval)
        elif right_eval is None:
            values[self.right].expect(values, left_eval)

        return values[self.left].eval(values) == values[self.right].eval(values)

    def expect(self, values, target):
        raise NotImplementedError


def run(filename):
    vals = dict()
    with open(filename, 'r') as f:
        for line in f.readlines():
            name, ops = line.split(":")
            tokens = ops.split()

            if name == "humn":
                vals[name] = Leaf(None)
            elif name == "root":
                vals[name] = Equality(tokens[0], tokens[2])
            elif len(tokens) == 1:
                vals[name] = Leaf(int(tokens[0]))
            elif tokens[1] == "+":
                vals[name] = Add(tokens[0], tokens[2])
            elif tokens[1] == "-":
                vals[name] = Subtract(tokens[0], tokens[2])
            elif tokens[1] == "*":
                vals[name] = Multiply(tokens[0], tokens[2])
            elif tokens[1] == "/":
                vals[name] = Divide(tokens[0], tokens[2])

    assert(vals["root"].eval(vals))
    return vals["humn"].eval(vals)


if __name__ == "__main__":
    print(run(sys.argv[1]))
