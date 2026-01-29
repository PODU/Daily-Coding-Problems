# Day 986: Longest consecutive run of 1-bits via the bit trick: n &= (n>>1) shrinks every
# run by one each step; iterations until n==0 equals the longest run length.
# Time: O(longest run), Space: O(1).


def longest_run(n):
    count = 0
    while n:
        count += 1
        n &= (n >> 1)
    return count


if __name__ == "__main__":
    print(longest_run(156))  # 156 = 10011100 -> 3
