# Day 138: Greedy on canonical US denominations {25,10,5,1}: take largest coin each step.
# Time O(D) where D = #denominations; Space O(1).


def min_coins(n):
    count = 0
    for c in (25, 10, 5, 1):
        count += n // c
        n %= c
    return count


if __name__ == "__main__":
    print(min_coins(16))  # 3
