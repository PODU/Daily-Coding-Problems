# Day 1301: Fewest jumps from 0 to N where the ith jump moves exactly i (left/right).
# Find smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even (flipping a jump changes sum by 2*val).


def min_jumps(N: int) -> int:
    N = abs(N)
    k = sum_ = 0
    while sum_ < N or (sum_ - N) % 2 != 0:
        k += 1
        sum_ += k
    return k


if __name__ == "__main__":
    print(min_jumps(3))  # 2
    print(min_jumps(2))  # 3
