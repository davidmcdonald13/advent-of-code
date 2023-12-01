import sys


def vprint(s=""):
    if len(sys.argv) >= 3 and sys.argv[2] == "-v":
        print(s)


class Node:
    def __init__(self, val, next, prev):
        self.val = val
        self.next = next
        self.prev = prev

    def __str__(self):
        return str(self.val)

    def __repr__(self):
        return str(self.val)


def linked_list_to_array(head):
    result = list()
    result.append(head)
    ptr = head.next
    while ptr != head:
        result.append(ptr)
        ptr = ptr.next
    return result


def run(filename):
    with open(filename, 'r') as f:
        order = [Node(int(x.strip()) * 811589153, None, None) for x in f.readlines()]

    order[0].prev = order[-1]
    order[-1].next = order[0]

    for i, node in enumerate(order[:-1]):
        node.next = order[i+1]
        order[i+1].prev = node

    zero_ptr = order[0]

    for round in range(10):
        print(f"starting round {round}")
        for node in order:
            if node.val == 0:
                zero_ptr = node
                continue

            move_count = node.val % (len(order) - 1)

            if move_count == 0:
                continue

            ptr = node
            for i in range(move_count):
                ptr = ptr.next

            # remove node from old place in list
            node.prev.next = node.next
            node.next.prev = node.prev

            # insert node into new place in list
            node.next = ptr.next
            node.prev = ptr

            ptr.next = node
            node.next.prev = node

    mixed = linked_list_to_array(zero_ptr)

    result = 0
    for target in [1000, 2000, 3000]:
        val = mixed[target % len(mixed)].val
        print(f"val #{target}: {val}")
        result += val

    return result


if __name__ == "__main__":
    print(run(sys.argv[1]))
