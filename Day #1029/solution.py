# Day 1029: Minimum coins for n cents with {1,5,10,25}. Greedy is optimal for
# this canonical US denomination set. Time O(#denominations), Space O(1).


def min_coins(n):
    count = 0
    for c in (25, 10, 5, 1):
        count += n // c
        n %= c
    return count


if __name__ == "__main__":
    print(min_coins(16))
