# Day 1743: Left fold (reduce): single linear pass applying combiner to accumulator. O(n) time, O(1) extra space.

def reduce(arr, combining_fn, initial_value):
    acc = initial_value
    for x in arr:
        acc = combining_fn(acc, x)
    return acc


def add(a, b):
    return a + b


def multiply(a, b):
    return a * b


def sum_list(arr):
    return reduce(arr, add, 0)


def main():
    print(sum_list([1, 2, 3, 4, 5]))
    print(reduce([1, 2, 3, 4], multiply, 1))


if __name__ == "__main__":
    main()
