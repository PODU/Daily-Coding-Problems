# Day 1568: Longest consecutive run of 1s using n &= (n<<1) bit trick. Time O(run length), space O(1).
def longest_run(n):
    count = 0
    while n:
        n &= (n << 1)
        count += 1
    return count


if __name__ == "__main__":
    print(longest_run(156))  # 10011100 -> 3
