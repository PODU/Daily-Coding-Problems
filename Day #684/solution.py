# Day 684: Custom reduce/fold (left to right). O(n) time, O(1) extra space.

def reduce(arr, fn, init):
    acc = init
    for x in arr:
        acc = fn(acc, x)
    return acc


if __name__ == "__main__":
    add = lambda a, b: a + b
    mul = lambda a, b: a * b
    print(reduce([1, 2, 3, 4], add, 0))  # 10
    print(reduce([1, 2, 3, 4], mul, 1))  # 24 (bonus)
