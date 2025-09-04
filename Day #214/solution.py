# Day 214: Longest consecutive run of 1s in binary representation.
# Approach: n &= (n<<1) collapses runs; count iterations. Time O(longest run), Space O(1).
def longest_run(n: int) -> int:
    count = 0
    while n:
        n &= (n << 1)
        count += 1
    return count


if __name__ == "__main__":
    print(longest_run(156))  # 156 = 10011100 -> 3
