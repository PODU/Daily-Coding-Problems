# Day 1326: Implement reduce/fold — fold an array left to right with a combining function and an initial value.
# O(n) calls to the combiner, O(1) extra space.

def reduce(lst, combine, init):
    acc = init
    for x in lst:
        acc = combine(acc, x)
    return acc


def add(a, b):
    return a + b


def total(lst):
    return reduce(lst, add, 0)


if __name__ == "__main__":
    print(total([1, 2, 3, 4, 5]))  # 15
