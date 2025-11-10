# Day 579: Min jumps: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even. Time O(sqrt(N)), Space O(1).
def min_jumps(N):
    n = abs(N)
    k = s = 0
    while s < n or (s - n) % 2 != 0:
        k += 1
        s += k
    return k


if __name__ == "__main__":
    print("Minimum jumps to reach 10:", min_jumps(10))
