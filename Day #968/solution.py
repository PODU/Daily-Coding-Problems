# Day 968: Fewest jumps from 0 to N where i-th jump moves +/- i.
# Approach: smallest k with sum 1..k >= |N| and (sum-|N|) even. Time O(sqrt(N)), Space O(1).


def min_jumps(n):
    n = abs(n)
    k = total = 0
    while total < n or (total - n) % 2 != 0:
        k += 1
        total += k
    return k


if __name__ == '__main__':
    print(min_jumps(10))  # 4  (1+2+3+4=10)
