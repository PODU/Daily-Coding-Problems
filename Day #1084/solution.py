# Day 1084: reduce/fold: apply combiner left-to-right starting from init. Time O(n), Space O(1).
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
