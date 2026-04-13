# Day 1355: Longest run of consecutive 1s in binary. Bit trick: n &= (n<<1) collapses runs; iterations = longest run. O(run) time, O(1) space.

def longest_run(n):
    count = 0
    while n:
        n &= (n << 1)
        count += 1
    return count


if __name__ == "__main__":
    print(longest_run(156))
