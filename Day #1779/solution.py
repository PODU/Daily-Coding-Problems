# Day 1779: Minimum coins for {1,5,10,25} via greedy (optimal for this canonical system).
# Time: O(1), Space: O(1).
def min_coins(n):
    return n // 25 + (n % 25) // 10 + (n % 25 % 10) // 5 + (n % 25 % 10 % 5)


if __name__ == "__main__":
    print(min_coins(16))
