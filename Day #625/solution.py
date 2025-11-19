# Day 625: Longest consecutive run of 1s in binary repr of n.
# Bit trick: n &= (n<<1) collapses runs; iterations = longest run. O(bits) time, O(1) space.

def longest_run(n: int) -> int:
    count = 0
    while n:
        n &= (n << 1)
        count += 1
    return count


if __name__ == "__main__":
    print(longest_run(156))  # 10011100 -> 3
