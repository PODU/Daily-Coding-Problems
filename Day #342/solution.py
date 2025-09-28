# Day 342: Custom left fold (reduce). acc=init; for each x: acc=f(acc,x). O(n) time, O(1) space.

def my_reduce(arr, f, init):
    acc = init
    for x in arr:
        acc = f(acc, x)
    return acc

def add(a, b):
    return a + b

def sum_list(lst):
    return my_reduce(lst, add, 0)

def main():
    print(sum_list([1, 2, 3, 4, 5]))

if __name__ == "__main__":
    main()
