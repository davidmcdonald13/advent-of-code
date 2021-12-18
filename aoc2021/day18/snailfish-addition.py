import json
import math
import sys

from queue import PriorityQueue

class Node:
    def __init__(self, left, right, val, is_left):
        self.left = left
        self.right = right
        self.val = val
        self.is_left = is_left
        self.parent = None

    def get_depth(self):
        if self.parent is None:
            return 0
        return 1 + self.parent.get_depth()

    def full_reduce(self):
        while self.do_explode() or self.do_split():
            continue

    def do_explode(self):
        if self.val is not None:
            return None
        elif self.need_to_explode():
            # explode this node
            left_dest = None
            prev = self
            curr = self.parent
            while curr:
                if prev.is_left:
                    prev = curr
                    curr = curr.parent
                else:
                    left_dest = curr.left.get_rightmost_val()
                    left_dest.val += self.left.val
                    break

            right_dest = None
            prev = self
            curr = self.parent
            while curr:
                if not prev.is_left:
                    prev = curr
                    curr = curr.parent
                else:
                    right_dest = curr.right.get_leftmost_val()
                    right_dest.val += self.right.val
                    break

            new_node = Node(None, None, 0, self.is_left)
            new_node.parent = self.parent
            return new_node
        else:
            # we need to traverse further down the tree
            left_result = self.left.do_explode()
            if left_result:
                self.left = left_result
                return self

            right_result = self.right.do_explode()
            if right_result:
                self.right = right_result
                return self

            return None

    def do_split(self):
        if self.val is not None:
            if self.val >= 10:
                # we need to split this node
                return self.split()
            else:
                return None
        else:
            # we need to traverse further down the tree
            left_result = self.left.do_split()
            if left_result:
                self.left = left_result
                return self

            right_result = self.right.do_split()
            if right_result:
                self.right = right_result
                return self

            return None


    def need_to_explode(self):
        return self.val is None and self.get_depth() >= 4 and (self.left.val is not None) and (self.right.val is not None)

    def split(self):
        left = Node(None, None, self.val // 2, True)
        right = Node(None, None, math.ceil(self.val / 2), False)

        new_node = Node(left, right, None, self.is_left)
        new_node.parent = self.parent

        left.parent = new_node
        right.parent = new_node

        return new_node

    def get_leftmost_val(self):
        if self.val is not None:
            return self
        return self.left.get_leftmost_val()

    def get_rightmost_val(self):
        if self.val is not None:
            return self
        return self.right.get_rightmost_val()

    def get_magnitude(self):
        if self.val is not None:
            return self.val
        return 3 * self.left.get_magnitude() + 2 * self.right.get_magnitude()

    def to_list(self):
        if self.val is not None:
            return self.val
        return [self.left.to_list(), self.right.to_list()]

def snailfish_addition(vals):
    reduced_value = vals[0]

    for val in vals[1:]:
        full_graph = Node(reduced_value, val, None, False)

        reduced_value.is_left = True
        reduced_value.parent = full_graph

        val.is_left = False
        val.parent = full_graph

        reduced_value = full_graph.full_reduce()

        if reduced_value is None:
            reduced_value = full_graph

    return reduced_value.get_magnitude()

def list_to_node(arr, depth=0, is_left=False):
    if type(arr) == list:
        left = list_to_node(arr[0], depth=depth+1, is_left=True)
        right = list_to_node(arr[1], depth=depth+1, is_left=False)

        result = Node(left, right, None, is_left)

        left.parent = result
        right.parent = result

        return result
    else:
        return Node(None, None, arr, is_left)

def get_graph(filename):
    inputs = list()
    with open(filename, 'r') as f:
        for line in f.readlines():
            clean_list = json.loads(line.strip())
            inputs.append(list_to_node(clean_list))
    return inputs

if __name__ == "__main__":
    inputs = get_graph(sys.argv[1])
    N = len(inputs)
    print("total sum:", snailfish_addition(inputs))

    # O(N^2) disk reads, but my code mutates the graphs...
    max_pair_sum = 0
    for i in range(N):
        for j in range(N):
            if i == j:
                continue
            inputs = get_graph(sys.argv[1])
            this_pair_sum = snailfish_addition([inputs[i], inputs[j]])
            max_pair_sum = max(max_pair_sum, this_pair_sum)
    print("max pair sum:", max_pair_sum)
